//! Outdoor worldspaces: the grid of squares, persistent objects, terrain
//! and weather, on the test world `testdata::outdoors` builds.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::{OUTDOOR_AMBIENT, OUTDOOR_BASE_HEIGHT, OUTDOOR_FOG, OUTDOOR_SKY, OUTDOOR_SUNLIGHT};
use world::land::{CELL_SIZE, GRID, QUARTER_GRID, SPACING};
use world::{find_worldspace, Land, LandTexture, WorldGrid};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::outdoors(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn finds_a_worldspaces_squares_and_its_persistent_cell() {
    let (_data, order) = order("grid");
    let world = find_worldspace(&order, "testworld").unwrap().unwrap();
    assert_eq!(world, FormId(0xC00));
    assert_eq!(find_worldspace(&order, "Test World").unwrap(), Some(world));
    let grid = WorldGrid::load(&order, world).unwrap();
    assert_eq!(grid.cell_at((0, 0)), Some(FormId(0xC10)));
    assert_eq!(grid.cell_at((1, 0)), Some(FormId(0xC20)));
    assert_eq!(grid.cell_at((0, 1)), None);
    // The persistent cell also says 0,0, but isn't the square's cell.
    assert_eq!(grid.persistent, Some(FormId(0xC01)));
    assert_eq!(grid.world.default_land_height, -2500.0);
    assert_eq!(grid.climate.as_ref().map(|c| c.sunrise), Some((6.0, 8.0)));
}

#[test]
fn a_square_holds_its_own_objects_and_the_persistent_ones_standing_in_it() {
    let (_data, order) = order("persistent");
    let grid = WorldGrid::load(&order, FormId(0xC00)).unwrap();
    let field = grid.load_square(&order, (0, 0)).unwrap().unwrap();
    let ids: Vec<u32> = field.objects.iter().map(|o| o.form_id.0).collect();
    // The persistent door to the shack stands here too.
    assert_eq!(ids, vec![0xC03, 0xC12]);
    let door = field.objects[0].teleport.expect("a load door");
    assert_eq!(door.door, FormId(0xD01));
    assert_eq!(door.position, [0.0, 100.0, 0.0]);
    let east = grid.load_square(&order, (1, 0)).unwrap().unwrap();
    let ids: Vec<u32> = east.objects.iter().map(|o| o.form_id.0).collect();
    assert_eq!(ids, vec![0xC02], "the persistent rock stands in square 1,0");
    assert!(grid.load_square(&order, (5, 5)).unwrap().is_none());
}

#[test]
fn map_markers_are_found_nearby_and_travelled_to() {
    use world::scripting::GameState;
    let (_data, order) = order("map");
    let grid = WorldGrid::load(&order, FormId(0xC00)).unwrap();
    let markers = world::map::markers(&order, grid.persistent.unwrap());
    assert_eq!(markers.len(), 1);
    let well = &markers[0];
    assert_eq!(well.reference, FormId(testdata::MAP_MARKER));
    assert_eq!(well.name, testdata::MAP_MARKER_NAME);
    assert_eq!(well.position, testdata::MAP_MARKER_AT);
    assert_eq!((well.radius, well.flags, well.kind), (500.0, 0x02, 1));
    assert_eq!(well.arrival, Some(FormId(0xC02)));
    // A marker isn't drawn.
    let field = grid.load_square(&order, (0, 0)).unwrap().unwrap();
    assert!(field.objects.iter().all(|o| o.form_id != well.reference));

    let mut state = GameState::new(&order);
    assert!(!world::map::shown(&state, well));
    assert!(!world::map::can_travel(&state, well));
    // 600 units away: not yet; 400 across but 900 up (measured in 3D, as
    // the game does): not yet; exactly on the radius (400 across, 300 up):
    // not yet (it must be inside); 400 across and 200 up: found.
    let [x, y, z] = testdata::MAP_MARKER_AT;
    let discover = |state: &mut GameState, at: [f32; 3]| {
        world::map::discover(&order, state, &markers, at).len()
    };
    assert_eq!(discover(&mut state, [x + 600.0, y, z]), 0);
    assert_eq!(discover(&mut state, [x, y + 400.0, z + 900.0]), 0);
    assert_eq!(discover(&mut state, [x, y + 400.0, z + 300.0]), 0);
    assert_eq!(discover(&mut state, [x, y + 400.0, z + 200.0]), 1);
    assert_eq!(discover(&mut state, [x, y, z]), 0);
    assert!(world::map::shown(&state, well) && world::map::can_travel(&state, well));
    // Worth `iXPRewardDiscoverMapMarker`, once.
    assert_eq!(world::experience::xp(&state), 10.0);
    // Announced by the HUD's quest text, not a corner message:
    // `sDiscoveredText` over the marker's name (`00779070`).
    let texts: Vec<_> = state
        .events
        .iter()
        .filter_map(|e| match e {
            world::scripting::Event::QuestText(world::quest_text::QuestText::Custom(c)) => {
                Some((c.title.clone(), c.subtitle.clone(), c.sound.clone()))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        texts,
        [(
            "You have discovered".to_string(),
            testdata::MAP_MARKER_NAME.to_string(),
            "UIPopUpQuestNew".to_string()
        )]
    );
    // (The only message is the experience, for the XP meter.)
    assert!(!state.events.iter().any(|e| matches!(
        e,
        world::scripting::Event::Message { text, .. } if text.contains(testdata::MAP_MARKER_NAME)
    )));

    // Found markers are kept in a save.
    let (mut state, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert!(world::map::can_travel(&state, well));

    // Not from an interior whose cell doesn't allow it.
    state.player_world = None;
    state.player_cell = Some(FormId(0xD00));
    assert!(world::map::travel(&order, &mut state, well).is_err());
    // From the worldspace: the trip takes the straight line at 308 units a
    // second as game time (× 30): 4500 units from 500,2000 → 0.12 hours.
    state.player_world = Some(FormId(0xC00));
    state.player_cell = Some(FormId(0xC01));
    state.player_position = Some([500.0, 2000.0, 1256.0]);
    let hours = world::map::travel_hours(&order, &state, [5000.0, 2000.0, 1256.0], FormId(0xC00));
    assert!(
        (hours - 4500.0 / 308.0 / 3600.0 * 30.0).abs() < 1e-4,
        "{hours}"
    );
    // Travelling puts the player at the arrival point: the rock in 1,0.
    assert_eq!(
        world::map::travel(&order, &mut state, well),
        Ok(FormId(0xC02))
    );
    assert_eq!(state.player_world, Some(FormId(0xC00)));
    assert_eq!(state.player_position, Some([5000.0, 2000.0, 1256.0]));
}

#[test]
fn the_weather_rolls_follows_regions_and_fades() {
    use world::weather::{roll, SkyMode, WeatherChance, WeatherState};
    let (_data, order) = order("weather-state");
    let world = FormId(0xC00);
    let (clear, storm) = (FormId(0xB00), FormId(testdata::OUTDOOR_STORM));
    // The roll: a global's value replaces the chance outright.
    let list = [
        WeatherChance {
            weather: clear,
            chance: 100,
            global: None,
        },
        WeatherChance {
            weather: storm,
            chance: 50,
            global: Some(FormId(testdata::OUTDOOR_STORMY)),
        },
    ];
    for dice in 0..200 {
        assert_eq!(roll(&list, |_| 0.0, dice), Some(clear));
    }
    // With the global at 1: the storm on 1 roll in 101.
    let storms = (0..101u64)
        .filter(|&d| roll(&list, |_| 1.0, d) == Some(storm))
        .count();
    assert_eq!(storms, 1);

    let mut w = WeatherState::default();
    let mut dice = 7u64;
    let mut next = move || {
        dice = dice.wrapping_mul(6364136223846793005).wrapping_add(1);
        dice >> 33
    };
    let global = |_: FormId| 0.0;
    // Arriving in square 0,0 (no weather region): the climate's pick, shown
    // at once.
    w.enter_cell(&order, FormId(0xC10), Some(world), [100.0, 100.0, 0.0]);
    assert_eq!(w.climate, Some(FormId(0xB10)));
    assert_eq!(w.region, None);
    w.update(&order, SkyMode::Exterior, 10.0, global, &mut next);
    assert_eq!((w.current, w.previous, w.fade), (Some(clear), None, 1.0));
    assert_eq!(
        w.region_weathers
            .get(&FormId(testdata::OUTDOOR_STORM_REGION)),
        Some(&storm)
    );
    // Into square 1,0, inside the storm region: its weather fades in over
    // 0.25 hours.
    w.enter_cell(&order, FormId(0xC20), Some(world), [5000.0, 100.0, 0.0]);
    assert_eq!(w.region, Some(FormId(testdata::OUTDOOR_STORM_REGION)));
    w.update(&order, SkyMode::Exterior, 10.5, global, &mut next);
    assert_eq!((w.current, w.previous), (Some(storm), Some(clear)));
    assert_eq!(w.fade, 0.0);
    w.update(&order, SkyMode::Exterior, 10.625, global, &mut next);
    assert!((w.fade - 0.5).abs() < 1e-4, "{}", w.fade);
    // Rainy (`DATA` 0x04) once its rain starts.
    assert!(w.precipitation(&order, 0x04));
    w.update(&order, SkyMode::Exterior, 10.8, global, &mut next);
    assert_eq!((w.current, w.previous, w.fade), (Some(storm), None, 1.0));
    // An interior showing the sky ignores region weathers: back to the
    // climate's pick, fading.
    w.update(&order, SkyMode::InteriorSky, 11.0, global, &mut next);
    assert_eq!((w.current, w.previous), (Some(clear), Some(storm)));
    // A script's override wins until released; `ForceWeather` is at once.
    w.force(clear, true, 12.0);
    w.update(&order, SkyMode::Exterior, 12.1, global, &mut next);
    assert_eq!((w.current, w.fade), (Some(clear), 1.0));
    // Released: the climate's pick (forcing also let go of the region until
    // the player next changes cell), then the region's storm again.
    w.forced = None;
    w.update(&order, SkyMode::Exterior, 12.2, global, &mut next);
    assert_eq!((w.current, w.previous), (Some(clear), None));
    w.enter_cell(&order, FormId(0xC20), Some(world), [5000.0, 100.0, 0.0]);
    w.update(&order, SkyMode::Exterior, 12.3, global, &mut next);
    assert_eq!((w.current, w.previous), (Some(storm), Some(clear)));
    // Kept in a save.
    let mut state = world::scripting::GameState::new(&order);
    state.weather = w.clone();
    let (loaded, _) = world::save::load(&world::save::save(&state, None)).unwrap();
    assert_eq!(loaded.weather.current, w.current);
    assert_eq!(loaded.weather.region_weathers, w.region_weathers);
}

#[test]
fn squares_are_lit_by_their_weather_by_day() {
    let (_data, order) = order("weather");
    let grid = WorldGrid::load(&order, FormId(0xC00)).unwrap();
    let field = grid.load_square(&order, (0, 0)).unwrap().unwrap();
    let l = field.info.lighting.expect("lit by the weather");
    assert_eq!(l.ambient, OUTDOOR_AMBIENT);
    assert_eq!(l.directional, OUTDOOR_SUNLIGHT);
    assert_eq!(l.fog_color, OUTDOOR_FOG);
    assert_eq!((l.fog_near, l.fog_far, l.fog_power), (100.0, 50_000.0, 0.5));
    assert_eq!(field.info.sky, Some(OUTDOOR_SKY));
    assert!(field.info.lighting_source.contains("TestWeather"));
    // Its clouds: four layers' textures, speeds and colours.
    assert_eq!(field.info.weather, Some(FormId(0xB00)));
    let w = world::weather::Weather::load(&order, FormId(0xB00)).unwrap();
    assert_eq!(w.cloud_textures[3].as_deref(), Some("sky\\TestClouds.dds"));
    assert_eq!(w.cloud_textures[0].as_deref(), Some("sky\\alpha.dds"));
    assert_eq!(w.cloud_speeds, [52, 0, 0, 65]);
    assert_eq!(
        w.cloud_color(3, world::weather::TimeOfDay::Day),
        Some(testdata::OUTDOOR_CLOUDS)
    );
    // At 10:00 the sun is in the east, 16° up (the game's path peaks at
    // 45° at 13:00).
    let toward = l.toward_directional();
    assert!(
        toward[0] > 0.9 && (0.2..0.35).contains(&toward[2]),
        "{toward:?}"
    );
}

#[test]
fn a_weathers_image_space_follows_the_hour() {
    use testdata::{OUTDOOR_DAY_TINT, OUTDOOR_NIGHT_TINT};
    let (_data, order) = order("weather-is");
    let grid = WorldGrid::load(&order, FormId(0xC00)).unwrap();
    let clock = world::weather::SkyClock::new(
        grid.climate.as_ref(),
        world::weather::SkySettings::load(&order),
    );
    let w = world::weather::Weather::load(&order, FormId(0xB00)).unwrap();
    assert_eq!(w.image_spaces[1], Some(FormId(0xB21)));
    assert_eq!(w.image_spaces[3], Some(FormId(0xB22)));
    let tint = |hour: f32| w.modifier_at(&order, &clock, hour).unwrap().tint;
    // The day's modifier is full at noon (the game's swap with high
    // noon's), the night's at night.
    let close = |a: [f32; 4], b: [f32; 4]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-5);
    assert!(close(tint(12.0), OUTDOOR_DAY_TINT));
    assert!(close(tint(23.0), OUTDOOR_NIGHT_TINT));
    // At 19:30 sunset (which names none: a blank modifier) has 0.8 and
    // night 0.2: the night's tint at a fifth of its amount.
    let dusk = tint(19.5);
    for k in 0..3 {
        assert!((dusk[k] - OUTDOOR_NIGHT_TINT[k]).abs() < 1e-5, "{dusk:?}");
    }
    assert!(
        (dusk[3] - 0.2 * OUTDOOR_NIGHT_TINT[3]).abs() < 1e-5,
        "{dusk:?}"
    );
    // The night's light: the weather's night colours (none set here: dark).
    let night = world::weather::exterior_lighting(&w, &clock, 23.0);
    assert_eq!(night.ambient, [0; 3]);
}

#[test]
fn reads_terrain_heights_colours_and_painted_layers() {
    let (_data, order) = order("land");
    let land = Land::of_cell(&order, FormId(0xC10)).unwrap().unwrap();
    assert_eq!(land.height(0, 0), Some(OUTDOOR_BASE_HEIGHT));
    assert_eq!(land.height(32, 0), Some(OUTDOOR_BASE_HEIGHT + 256.0));
    assert_eq!(land.height(5, 20), Some(OUTDOOR_BASE_HEIGHT + 40.0));
    let colors = land.colors.as_ref().unwrap();
    assert_eq!(colors[0], [128, 64, 32]);
    assert_eq!(colors.len(), GRID * GRID);
    assert_eq!(land.normals.as_ref().unwrap()[100], [0.0, 0.0, 1.0]);

    let sw = &land.quarters[0];
    assert_eq!(sw.base, Some(FormId(0xA00)));
    let layers: Vec<(u16, u32)> = sw.layers.iter().map(|l| (l.layer, l.texture.0)).collect();
    assert_eq!(
        layers,
        vec![(0, 0xA01), (1, 0xA00)],
        "sorted by layer number"
    );
    assert_eq!(sw.layers[0].opacity[0], 1.0);
    assert_eq!(sw.layers[0].opacity[QUARTER_GRID + 1], 0.5);
    assert_eq!(sw.layers[0].opacity[2], 0.0);
    // The other quarters name nothing: the default land texture.
    assert_eq!(land.quarters[3].base, None);

    let road = LandTexture::load(&order, FormId(0xA01)).unwrap();
    assert_eq!(road.diffuse.as_deref(), Some("Test\\Road.dds"));
    assert_eq!(road.normal, None);
    let dirt = LandTexture::load(&order, FormId(0xA00)).unwrap();
    assert_eq!(dirt.normal.as_deref(), Some("Test\\Dirt_n.dds"));
    assert_eq!(dirt.specular_exponent, Some(30));
}

#[test]
fn terrain_quarters_become_meshes_in_world_space() {
    let (_data, order) = order("mesh");
    let land = Land::of_cell(&order, FormId(0xC20)).unwrap().unwrap();
    // Square 1,0's north-east quarter.
    let origin = [CELL_SIZE, 0.0];
    let mesh = land.quarter_mesh(3, origin).unwrap();
    let n = QUARTER_GRID;
    assert_eq!(mesh.positions.len(), n * n);
    assert_eq!(mesh.indices.len(), (n - 1) * (n - 1) * 6);
    let half = (QUARTER_GRID - 1) as f32 * SPACING;
    assert_eq!(
        mesh.positions[0],
        [CELL_SIZE + half, half, OUTDOOR_BASE_HEIGHT + 256.0]
    );
    assert_eq!(
        mesh.positions[n * n - 1],
        [2.0 * CELL_SIZE, CELL_SIZE, OUTDOOR_BASE_HEIGHT + 256.0]
    );
    // Triangles face up (counter-clockwise from above).
    let [a, b, c] = [0, 1, 2].map(|k| mesh.positions[usize::from(mesh.indices[k])]);
    let cross_z = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
    assert!(cross_z > 0.0);
    // No layers: the base shows everywhere.
    assert_eq!(mesh.textures, vec![None]);
    assert!(mesh.weights.iter().all(|w| w == &vec![1.0]));

    // Square 0,0's painted quarter: at its first point the road (layer
    // 0, opacity 1) and dirt (layer 1, 0.5) add up past 1, so they share
    // it two to one and the base gets nothing (the game's rule).
    let field = Land::of_cell(&order, FormId(0xC10)).unwrap().unwrap();
    let mesh = field.quarter_mesh(0, [0.0, 0.0]).unwrap();
    assert_eq!(
        mesh.textures,
        vec![
            Some(FormId(0xA00)),
            Some(FormId(0xA01)),
            Some(FormId(0xA00))
        ]
    );
    let w = &mesh.weights[0];
    assert_eq!(w[0], 0.0);
    assert!(
        (w[1] - 2.0 / 3.0).abs() < 1e-6 && (w[2] - 1.0 / 3.0).abs() < 1e-6,
        "{w:?}"
    );
    assert_eq!(mesh.colors[0], [128.0 / 255.0, 64.0 / 255.0, 32.0 / 255.0]);
}

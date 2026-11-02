//! Grass on the outdoor test world (`testdata::outdoors`): its dirt texture
//! lists one grass, painted over square 0,0's south-west quarter.

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::{OUTDOOR_BASE_HEIGHT, OUTDOOR_GRASS};
use world::grass::{square_grass, texture_grasses, water_level, Grass, GrassSettings, NO_WATER};
use world::WorldGrid;

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = testdata::outdoors(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn land_textures_list_their_grasses() {
    let (_data, order) = order("grass-records");
    assert_eq!(
        texture_grasses(&order, FormId(0xA00)),
        vec![FormId(OUTDOOR_GRASS)]
    );
    assert!(texture_grasses(&order, FormId(0xA01)).is_empty());
    let g = Grass::load(&order, FormId(OUTDOOR_GRASS)).unwrap();
    assert_eq!(g.model.as_deref(), Some("Test\\Grass.nif"));
    assert_eq!((g.density, g.min_slope, g.max_slope), (100, 0, 90));
    assert_eq!((g.position_range, g.wave_period), (16.0, 10.0));
    assert!(g.fit_to_slope() && g.uniform_scaling());
}

#[test]
fn the_dirt_grows_grass_on_every_candidate_of_its_quarter() {
    let (_data, order) = order("grass-square");
    let grid = WorldGrid::load(&order, FormId(0xC00)).unwrap();
    let settings = GrassSettings {
        texture_pct_threshold: 0.0,
        ..GrassSettings::default()
    };
    // No water flag on the square.
    assert_eq!(water_level(&order, &grid, FormId(0xC10)), NO_WATER);
    let batches = square_grass(&order, &grid, (0, 0), &settings).unwrap();
    assert_eq!(batches.len(), 1);
    let b = &batches[0];
    assert_eq!(b.grass.form_id, FormId(OUTDOOR_GRASS));
    // Full density: 16 spots × 6 × 6 candidates, all kept.
    assert_eq!(b.instances.len(), 16 * 36);
    for i in &b.instances {
        let [x, y, z, _] = i.data;
        // In the quarter, give or take the position range.
        assert!((-16.0..2048.0 + 16.0).contains(&x), "{x}");
        assert!((-16.0..2048.0 + 16.0).contains(&y), "{y}");
        // On the ramp, rising 8 units per 128 east (west and south of the
        // square there's no terrain: the game's −2048).
        if x >= 0.0 && y >= 0.0 {
            let ground = OUTDOOR_BASE_HEIGHT + x.floor() / 16.0;
            assert!((z - ground).abs() < 2.0, "{x} {z} {ground}");
        } else {
            assert!((z - -2047.03).abs() < 0.05, "{z}");
        }
        // Colour range 0.5: 0.25 to 0.375 of the ground's luminance.
        assert!(i.brightness <= 0.375 + 1e-4);
    }
    // The same every time.
    assert_eq!(
        square_grass(&order, &grid, (0, 0), &settings).unwrap(),
        batches
    );
    // Square 1,0 is painted with nothing: no grass.
    assert!(square_grass(&order, &grid, (1, 0), &settings)
        .unwrap()
        .is_empty());
}

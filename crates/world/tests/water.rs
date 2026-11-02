//! Water read from a plugin built from scratch (`testdata::water::lake`).

use esm::{ActivePlugins, FormId, LoadOrder};
use testdata::water as lake;
use world::water::{cell_water, placed_flags, PlaceableWater, WaterType, WorldWater};

fn order(tag: &str) -> (testdata::TempData, LoadOrder) {
    let data = lake::lake(tag);
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    (data, order)
}

#[test]
fn a_cells_water_is_its_own_or_the_worldspaces() {
    let (_data, order) = order("cells");
    // Its own height and type.
    let shore = cell_water(&order, FormId(lake::SHORE)).unwrap().unwrap();
    assert_eq!(shore.height, lake::SHORE_HEIGHT);
    assert_eq!(shore.water_type, Some(FormId(lake::POND_WATER)));
    assert!(!shore.default_height);
    // `XCLW` holding the largest float: the worldspace's height and type.
    let dry = cell_water(&order, FormId(lake::DRY_LAND)).unwrap().unwrap();
    assert_eq!(dry.height, lake::DEFAULT_HEIGHT);
    assert_eq!(dry.water_type, Some(FormId(lake::WATER)));
    assert!(dry.default_height);
    // No "has water" flag: none, whatever `XCLW` says.
    assert_eq!(cell_water(&order, FormId(lake::NO_WATER)).unwrap(), None);
    // Interiors never have a cell's water.
    assert_eq!(cell_water(&order, FormId(lake::BATH)).unwrap(), None);
}

#[test]
fn water_types_and_the_worldspaces_water_are_read() {
    let (_data, order) = order("types");
    let w = WaterType::load(&order, FormId(lake::WATER)).unwrap();
    assert_eq!(w.label(), "TestWater");
    assert_eq!(w.noise.as_deref(), Some(lake::TYPE_NOISE));
    assert_eq!(w.opacity, lake::OPACITY);
    let v = w.visual.unwrap();
    assert_eq!(&v.shallow[..3], &lake::SHALLOW);
    assert_eq!(&v.deep[..3], &lake::DEEP);
    assert_eq!(v.fog_far, 500.0);
    assert_eq!(v.reflection_multiplier(), 2.0);
    assert_eq!(v.wind_directions, [0.0, 90.0, 180.0]);
    assert!(WaterType::load(&order, FormId(lake::POOL)).is_none());

    let world = WorldWater::load(&order, FormId(lake::WORLD)).unwrap();
    assert_eq!(world.default_height, lake::DEFAULT_HEIGHT);
    assert_eq!(world.water_type, Some(FormId(lake::WATER)));
    assert_eq!(world.noise.as_deref(), Some(lake::WORLD_NOISE));
    assert_eq!(world.lod_water_type, Some(FormId(lake::POND_WATER)));
    assert_eq!(world.lod_height, Some(lake::LOD_WATER_HEIGHT));
}

#[test]
fn placed_water_names_its_model_flags_and_type() {
    let (_data, order) = order("placed");
    let p = PlaceableWater::load(&order, FormId(lake::POOL)).unwrap();
    assert_eq!(p.model.as_deref(), Some("Water\\Pool.nif"));
    assert_eq!(p.flags, lake::POOL_FLAGS);
    assert_eq!(p.water_type, Some(FormId(lake::POND_WATER)));
    assert_eq!(p.flags & placed_flags::REFLECTS, 0);
    assert_ne!(p.flags & placed_flags::REFRACTS, 0);
    assert_ne!(p.flags & placed_flags::DEPTH, 0);
    // The pool is among the square's placed objects.
    let grid = world::WorldGrid::load(&order, FormId(lake::WORLD)).unwrap();
    let square = grid.load_square(&order, (0, 0)).unwrap().unwrap();
    assert!(square
        .objects
        .iter()
        .any(|o| o.form_id == FormId(lake::POOL_REF)
            && world::water::is_placeable_water(o.base_type)));
}

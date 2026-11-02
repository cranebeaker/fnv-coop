//! The water a place gives the viewer (`testdata::water::lake`).

use cellview::{Game, Options};
use testdata::water as lake;

fn game(tag: &str) -> (testdata::TempData, Game) {
    let data = lake::lake(tag);
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    (data, game)
}

#[test]
fn a_square_gets_its_own_water_and_its_placed_water() {
    let (_data, game) = game("square");
    let grid = game.world("TestLake").unwrap();
    let scene = game.load_square(&grid, (0, 0)).unwrap().unwrap();
    assert_eq!(scene.water.len(), 2, "{:?}", scene.notes);

    // The square's own water: the whole square at its height, its type,
    // the worldspace's noise texture.
    let own = &scene.water[0];
    assert_eq!(own.height, lake::SHORE_HEIGHT);
    assert_eq!(own.water_type.label(), "TestPondWater");
    assert_eq!(own.positions[0], [0.0, 0.0, lake::SHORE_HEIGHT]);
    assert_eq!(own.positions[2], [4096.0, 4096.0, lake::SHORE_HEIGHT]);
    assert_eq!(own.indices, vec![0, 1, 2, 0, 2, 3]);
    let noise = &scene.textures[own.noise.unwrap()];
    assert_eq!(noise.path, "textures\\water\\worldnoise.dds");
    assert!(noise.linear);
    // A cell's water reflects, refracts and has depth; not at the
    // worldspace's own water height: only the sky in its reflection.
    assert!(own.reflections && own.refractions && own.depth);
    assert!(!own.reflects_scene && !own.interior);

    // The pool: its model placed (scale 0.5, a quarter turn), wound to
    // face up, its flags (no reflection) and type.
    let pool = &scene.water[1];
    assert_eq!(pool.reference, lake::POOL_REF);
    assert!(!pool.reflections && pool.refractions && pool.depth);
    assert_eq!(pool.scale, lake::POOL_SCALE);
    assert!((pool.height - lake::POOL_AT[2]).abs() < 1e-3);
    let half = lake::POOL_HALF_SIZE * lake::POOL_SCALE;
    let xs: Vec<f32> = pool.positions.iter().map(|p| p[0]).collect();
    let lo = xs.iter().copied().fold(f32::MAX, f32::min);
    assert!((lo - (lake::POOL_AT[0] - half)).abs() < 1e-3, "{xs:?}");
    // Turned a quarter clockwise: the model's x axis points south.
    let [x_axis, _] = pool.axes;
    assert!(
        x_axis[0].abs() < 1e-5 && (x_axis[1] + 1.0).abs() < 1e-5,
        "{x_axis:?}"
    );
    for t in pool.indices.chunks(3) {
        let [a, b, c] = [t[0], t[1], t[2]].map(|i| pool.positions[usize::from(i)]);
        let up = (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]);
        assert!(up > 0.0);
    }
    // The pool's model isn't drawn as an ordinary mesh.
    assert!(scene.meshes.iter().all(|m| !m.name.contains("pool.nif")));
}

#[test]
fn water_under_the_ground_or_without_the_flag_isnt_drawn() {
    let (_data, game) = game("dry");
    let grid = game.world("TestLake").unwrap();
    // Water at the default height, all of it under the ground.
    let dry = game.load_square(&grid, (1, 0)).unwrap().unwrap();
    assert!(dry.water.is_empty());
    let none = game.load_square(&grid, (2, 0)).unwrap().unwrap();
    assert!(none.water.is_empty());
}

#[test]
fn water_at_the_distant_water_height_reflects_the_scene() {
    let (_data, game) = game("sea");
    let grid = game.world("TestLake").unwrap();
    let scene = game.load_square(&grid, (3, 0)).unwrap().unwrap();
    assert_eq!(scene.water.len(), 1);
    let sea = &scene.water[0];
    assert_eq!(sea.height, lake::LOD_WATER_HEIGHT);
    assert!(sea.reflections && sea.reflects_scene && !sea.interior);
}

#[test]
fn distant_water_comes_from_the_chunks_unshaded_shape_split_by_cell() {
    let (_data, game) = game("lod");
    let pieces = cellview::water::lod_water(&game.assets, "TestLake", (0, 0));
    // The land (a shape with a property) isn't water; the water shape's
    // two triangles lie over squares 1,0 and 2,0, moved by its own node.
    assert_eq!(pieces.len(), 2, "{pieces:?}");
    assert_eq!(pieces[0].cell, (1, 0));
    assert_eq!(pieces[1].cell, (2, 0));
    for p in &pieces {
        assert_eq!(p.height, lake::LOD_WATER_HEIGHT);
        assert_eq!(p.indices.len(), 3);
        assert!(p
            .positions
            .iter()
            .all(|q| q[0] >= 6144.0 && q[0] <= 10240.0));
    }
    assert!(cellview::water::lod_water(&game.assets, "TestLake", (4, 0)).is_empty());
    // Its type: the worldspace's distant water type.
    let t = cellview::water::lod_water_type(&game.order, esm::FormId(lake::WORLD)).unwrap();
    assert_eq!(t.label(), "TestPondWater");
}

#[test]
fn inside_only_placed_water_and_it_reflects_the_room() {
    let (_data, game) = game("bath");
    let bath = game.find_cell("TestBath").unwrap();
    let scene = game.load_cell(bath).unwrap();
    assert_eq!(scene.water.len(), 1);
    let pool = &scene.water[0];
    assert!(pool.interior && pool.reflects_scene);
    // No worldspace: the type's own noise texture (the pool's type names
    // none).
    assert_eq!(pool.noise, None);
}

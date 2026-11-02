//! Grass on the outdoor test world, ready to draw: the blades of square
//! 0,0 and the grass model copied once per blade.

use cellview::grass::grass_mesh;
use cellview::{Game, Options};
use testdata::OUTDOOR_GRASS;

#[test]
fn a_squares_grass_becomes_one_mesh_per_grass() {
    let data = testdata::outdoors("cellview-grass");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    let grid = game.world("TestWorld").unwrap();
    let batches = game.square_grass(&grid, (0, 0)).unwrap();
    assert_eq!(batches.len(), 1);
    let batch = &batches[0];
    assert_eq!(batch.grass.form_id.0, OUTDOOR_GRASS);
    let blades = batch.instances.len();
    assert!(blades > 0);

    let model = game
        .grass_model(batch.grass.model.as_deref().unwrap())
        .expect("the grass model");
    assert_eq!(model.path, "meshes\\test\\grass.nif");
    assert_eq!(model.positions.len(), 4);
    assert_eq!(model.indices.len(), 6);
    assert_eq!(
        model.texture.as_ref().map(|t| t.path.as_str()),
        Some("textures\\test\\grass.dds")
    );
    // No vertex colours: white, swaying fully.
    assert_eq!(model.colors[0], [1.0; 4]);
    assert!((model.radius - (16f32.powi(2) + 64f32.powi(2)).sqrt()).abs() < 1e-3);

    let mesh = grass_mesh(batch, &model, 125.0);
    assert_eq!(mesh.positions.len(), blades * 4);
    assert_eq!(mesh.instances.len(), blades * 4);
    assert_eq!(mesh.indices.len(), blades * 6);
    // Each copy's vertices carry their own blade and index their own copy.
    assert_eq!(mesh.instances[4], batch.instances[1].data);
    assert_eq!(&mesh.indices[6..9], &[4, 5, 6]);
    // The bounds hold every blade with room for its size and sway.
    let (lo, hi) = mesh.bounds;
    for i in &batch.instances {
        for axis in 0..3 {
            assert!(lo[axis] < i.data[axis] - 60.0 && i.data[axis] + 60.0 < hi[axis]);
        }
    }
}

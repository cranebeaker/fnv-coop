//! Outdoor collision as the viewer gets it.

use cellview::{Game, Options};
use testdata::OUTDOOR_BASE_HEIGHT;

#[test]
fn the_ground_stands_out_from_the_terrain_by_the_height_fields_radius() {
    let data = testdata::outdoors("collision");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    let grid = game.world("TestWorld").unwrap();
    let scene = game.load_square(&grid, (0, 0)).unwrap().unwrap();
    let x = 10.0 * 128.0;
    let (_, t) = scene
        .collision
        .raycast([x, 3000.0, 5000.0], [0.0, 0.0, -1.0], 10_000.0)
        .expect("the ground is solid");
    // The game's terrain triangles have a radius of 0.5 Havok units.
    assert_eq!(scene.collision.shell(t), physics::TERRAIN_SHELL);
    assert!((physics::TERRAIN_SHELL - 3.4996).abs() < 1e-3);
    // Standing there, the feet are that radius and the proxy's keep
    // distance (0.05 Havok units) above the terrain (the hull's surface is
    // at the feet).
    let shape = physics::CharacterShape::PLAYER;
    let ground = OUTDOOR_BASE_HEIGHT + 80.0;
    let mut p = physics::Character::new([x, 3000.0, ground + 20.0]);
    for _ in 0..120 {
        p.update(&scene.collision, &shape, [0.0, 0.0], 1.0 / 60.0);
    }
    let expected = ground + physics::TERRAIN_SHELL + 0.05 * physics::HAVOK_UNIT;
    assert!(
        p.on_ground && (p.feet[2] - expected).abs() < 0.5,
        "{:?} vs {expected}",
        p.feet
    );
}

//! Loads squares of the outdoor test world, as the viewer streams them.

use cellview::{Game, Options};
use testdata::{OUTDOOR_BASE_HEIGHT, OUTDOOR_SKY};

#[test]
fn loads_a_square_with_its_terrain_textures_and_collision() {
    let data = testdata::outdoors("cellview");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    let grid = game.world("TestWorld").unwrap();
    let scene = game.load_square(&grid, (0, 0)).unwrap().unwrap();

    // The rock and the door, and the terrain in four quarters.
    assert_eq!(scene.draws.len(), 2);
    assert_eq!(scene.terrain.len(), 4);
    let sw = &scene.terrain[0];
    assert_eq!(sw.textures.len(), 3, "base and two layers");
    let path = |i: Option<usize>| i.map(|i| scene.textures[i].path.clone());
    assert_eq!(
        path(sw.textures[0].0).as_deref(),
        Some("textures\\test\\dirt.dds")
    );
    assert_eq!(
        path(sw.textures[1].0).as_deref(),
        Some("textures\\test\\road.dds")
    );
    // Dirt has a normal map, the road's set names none: the default's.
    assert!(scene.textures[sw.textures[0].1.unwrap()].linear);
    assert_eq!(
        path(sw.textures[1].1).as_deref(),
        Some("textures\\landscape\\dirtwasteland01_n.dds")
    );
    // Road 1 and dirt 0.5 add up past 1: shared two to one, no base.
    let w = &sw.weights[0];
    assert_eq!((w[0], w[3]), (0.0, 0.0));
    assert!(
        (w[1] - 2.0 / 3.0).abs() < 1e-6 && (w[2] - 1.0 / 3.0).abs() < 1e-6,
        "{w:?}"
    );
    // A quarter painted with nothing gets the default land texture.
    let ne = &scene.terrain[3];
    assert_eq!(
        path(ne.textures[0].0).as_deref(),
        Some("textures\\landscape\\dirtwasteland01.dds")
    );
    assert_eq!(ne.weights[0][0], 1.0);
    assert_eq!(
        scene.sky.map(|s| s[1]),
        Some(OUTDOOR_SKY[1].map(|c| f32::from(c) / 255.0))
    );

    // The player walks on the terrain: straight down onto the ramp, 8 units
    // higher per grid step east.
    let x = 10.0 * 128.0;
    let (d, _) = scene
        .collision
        .raycast([x, 3000.0, 5000.0], [0.0, 0.0, -1.0], 10_000.0)
        .expect("the ground is solid");
    let ground = 5000.0 - d;
    assert!(
        (ground - (OUTDOOR_BASE_HEIGHT + 80.0)).abs() < 0.01,
        "{ground}"
    );

    // The door leads into the shack.
    assert_eq!(scene.doors.len(), 1);
    let door = &scene.doors[0];
    assert_eq!(door.cell, Some(0xD00));
    assert!(door.interior);
    assert_eq!(door.world, None);
    assert_eq!(door.cell_label, "TestShack");
}

#[test]
fn distant_land_chunks_are_put_in_the_world_by_their_top_node() {
    let data = testdata::outdoors("lod");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    let chunk = game.lod_chunk("TestWorld", (0, 0)).expect("a chunk at 0,0");
    // Moved once by the top node, not twice.
    assert_eq!(chunk.positions[0], [100.0, 200.0, 950.0]);
    assert_eq!(chunk.indices.len(), 6);
    let diffuse = chunk.diffuse.expect("the chunk's texture");
    assert_eq!(
        diffuse.path,
        "textures\\landscape\\lod\\testworld\\diffuse\\testworld.n.level4.x0.y0.dds"
    );
    assert!(game.lod_chunk("TestWorld", (4, 0)).is_none());
    // Without geomorph heights of its own, a chunk morphs toward its own.
    assert_eq!(chunk.morph_heights, vec![950.0; 4]);
    assert_eq!(chunk.level, 4);
}

#[test]
fn distant_land_reads_its_quadtree_and_geomorph_heights() {
    let data = testdata::outdoors("lod-levels");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    let settings = game.lod_settings("TestWorld").expect("the .dlodsettings");
    assert_eq!(
        (settings.min_level, settings.max_level, settings.root_level),
        (4, 8, 8)
    );
    assert_eq!(settings.root, (0, 0));
    assert!(game.lod_settings("Nowhere").is_none());
    let node = world::lod::LodNode {
        level: 8,
        x: 0,
        y: 0,
    };
    let chunk = game.lod_land("TestWorld", node).expect("the level-8 chunk");
    assert_eq!((chunk.level, chunk.cell), (8, (0, 0)));
    // Its own heights 900, morph heights 800, both moved 50 up by the top
    // node (texcoord1 is in the shape's own space, as its heights are).
    assert_eq!(chunk.positions[0][2], 950.0);
    assert_eq!(chunk.morph_heights, vec![850.0; 4]);
    // The exe's defaults where the INI says nothing.
    let terrain = game.terrain_settings();
    assert_eq!(terrain.morph_start_mult, 0.7);
    assert_eq!(terrain.texture_fade_ms, 1000.0);
}

#[test]
fn an_interior_door_can_lead_outdoors() {
    let data = testdata::outdoors("door-out");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    let shack = game.find_cell("TestShack").unwrap();
    let scene = game.load_cell(shack).unwrap();
    assert_eq!(scene.doors.len(), 1);
    let door = &scene.doors[0];
    // The far side is the persistent door in the worldspace.
    assert!(!door.interior);
    assert_eq!(door.world, Some(0xC00));
    assert_eq!(door.cell_label, "TestWorld");
    assert_eq!(door.arrive, [2000.0, 1900.0, 1128.0]);
    assert_eq!(door.arrive_heading, 1.5);
}

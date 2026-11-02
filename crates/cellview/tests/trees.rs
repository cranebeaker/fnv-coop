//! Trees on the test tree world: the placed tree found on its square with
//! its seed, its record read, its `.spt` grown into the game's meshes with
//! the record's textures.

use cellview::{Game, Options};
use testdata::trees::{TREE_AT, TREE_BASE, TREE_REF, TREE_SEED};

fn game(tag: &str) -> (testdata::TempData, Game) {
    let data = testdata::trees::trees(tag);
    let options = Options {
        official: true,
        ..Options::default()
    };
    let game = Game::open(data.path(), &options).unwrap();
    (data, game)
}

#[test]
fn a_placed_tree_grows_with_its_records_seed() {
    let (_data, game) = game("cellview");
    let grid = game.world("TreeWorld").unwrap();
    let trees = game
        .square_trees(&grid, (0, 0), &world::Disabled::new())
        .unwrap();
    assert_eq!(trees.len(), 1);
    let t = &trees[0];
    assert_eq!((t.reference.0, t.base.0), (TREE_REF, TREE_BASE));
    assert_eq!(t.position, TREE_AT);
    assert_eq!(t.scale, 1.5);
    assert_eq!(t.xsed, Some(0));

    let base = world::tree::TreeBase::load(&game.order, t.base).unwrap();
    assert_eq!(base.seeds, vec![TREE_SEED as u32]);
    assert_eq!(base.seed(t.xsed), TREE_SEED);
    assert_eq!(base.spt_path(), "trees\\testtree.spt");
    assert_eq!(
        (base.curvature, base.rock_speed, base.rustle_speed),
        (2.0, 0.07, 0.35)
    );

    let settings = game.tree_settings();
    // The exe's defaults (no INI here): 2048 and 16384 × 0.5.
    assert_eq!((settings.near, settings.far), (1024.0, 8192.0));
    let model = game
        .tree_model(t.base, TREE_SEED, &settings)
        .expect("the tree");
    assert_eq!(model.branches.len(), 2);
    assert_eq!(model.leaves.len(), 2);
    assert!(!model.branches[0].strip.is_empty());
    assert!(!model.leaves[0].indices.is_empty());
    assert_eq!(model.curvature, 2.0);
    assert_eq!((model.rock_amount, model.rustle_amount), (0.07, 1.0));
    assert_eq!(
        model.branch_texture.as_ref().map(|t| t.path.as_str()),
        Some("textures\\trees\\branches\\testbark.dds")
    );
    assert!(model.branch_normal_map.is_some());
    assert_eq!(
        model.leaf_texture.as_ref().map(|t| t.path.as_str()),
        Some("textures\\trees\\leaves\\testleaves.dds")
    );
    assert_eq!(model.leaf_base.len(), cellview::tree::LEAF_BASE_SIZE);
    assert!(model.radius > 50.0);
    // Placed at its spot, scaled.
    let m = t.matrix();
    assert_eq!([m[12], m[13], m[14]], TREE_AT);
    assert!((m[0] - 1.5).abs() < 1e-6);
}

#[test]
fn trees_are_not_read_as_nif_models() {
    let (_data, game) = game("cellview-notes");
    let grid = game.world("TreeWorld").unwrap();
    let scene = game
        .load_square(&grid, (0, 0))
        .unwrap()
        .expect("the square");
    let notes = &scene.notes;
    assert!(
        !notes.iter().any(|n| n.contains("missing model")),
        "{notes:?}"
    );
}

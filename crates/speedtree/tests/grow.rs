//! A small tree built token by token (`testdata::trees::spt_file`), read,
//! grown and turned into the game's meshes.

use speedtree::lod::{lod_level, visible, LeafFade, LodLimits};
use speedtree::mesh::{branch_meshes, leaf_base, leaf_meshes, mixed_leaf_textures, MeshSettings};
use speedtree::tree::{grow, Settings};
use speedtree::SptFile;
use testdata::trees::{spt_file, TREE_SEED};

fn settings(spt: &SptFile) -> Settings {
    let mut s = Settings::from_file(spt);
    s.size = spt.size * 10.0;
    s.size_variance = spt.size_variance * 10.0;
    s.rocking_groups = 2;
    s
}

#[test]
fn reads_the_files_blocks() {
    let spt = SptFile::parse(&spt_file()).unwrap();
    assert_eq!(spt.branch_texture, "TestBark.tga");
    assert_eq!(spt.seed, Some(TREE_SEED));
    assert_eq!((spt.size, spt.size_variance), (20.0, 2.0));
    assert_eq!(spt.levels.len(), 3);
    assert_eq!(spt.levels[0].cross_sections, 6);
    assert_eq!(spt.leaves.textures.len(), 2);
    assert_eq!(spt.leaves.textures[1].origin, [0.45, 0.3, 0.0]);
    assert_eq!(spt.lod.branch_lods, 2);
    assert_eq!(spt.lod.leaf_lods, Some(2));
    assert_eq!(spt.billboards.as_ref().map(|b| b.a.len()), Some(2));
    assert_eq!((spt.wind_3c, spt.wind_40), (Some(1.0), Some(0.07)));
    assert_eq!(spt.srand, 444);
    assert!(spt.wind_per_level);
    assert_eq!(spt.trailing, None);
    // The two leaf textures are different files.
    assert!(mixed_leaf_textures(&spt));
}

#[test]
fn the_same_seed_grows_the_same_tree() {
    let spt = SptFile::parse(&spt_file()).unwrap();
    let s = settings(&spt);
    let a = grow(&spt, &s);
    let b = grow(&spt, &s);
    assert_eq!(a, b);
    // Grown to the file's size × 10 within the variance.
    assert!((180.0..=220.0).contains(&a.size), "{}", a.size);
    assert!(a.branches.len() > 3);
    assert!(!a.leaves.is_empty());
    let mut other = s.clone();
    other.seed += 1;
    assert_ne!(grow(&spt, &other).geometry.positions, a.geometry.positions);
}

#[test]
fn meshes_follow_the_games_layout() {
    let spt = SptFile::parse(&spt_file()).unwrap();
    let s = settings(&spt);
    let tree = grow(&spt, &s);
    let ms = MeshSettings {
        rocking_groups: 2,
        wind_matrices: (0, 4),
        leaf_lod_step: spt.lod.value_24,
    };
    let leaves = leaf_meshes(&spt, &tree, &ms);
    assert_eq!(leaves.len(), 2);
    for (lod, m) in leaves.iter().enumerate() {
        let n = tree.leaf_lods[lod].len();
        assert_eq!(m.positions.len(), n * 4);
        assert_eq!(m.indices.len(), n * 6);
        // The second level's cards are × (1 + 9009).
        let scale = if lod == 0 { 1.0 } else { 1.6 };
        assert!(m.blend.iter().all(|b| (b[3] - scale).abs() < 1e-6));
        for (k, b) in m.blend.iter().enumerate() {
            // Corner (k + 2) & 3 of its card, the brightness below 1.
            let corner = b[2].trunc() as usize;
            assert_eq!(corner % 4, ((k % 4) + 2) & 3);
            assert!(b[2].fract() < 1.0);
            assert_eq!(b[1] % 4.0, 0.0);
        }
        // v turned round.
        assert!(m.uvs.iter().all(|uv| uv[1] <= 0.0));
    }
    assert!(leaves[1].positions.len() < leaves[0].positions.len());
    // Per map (two per texture) and rocking group, four corners.
    assert_eq!(leaf_base(&tree, 2).len(), 2 * 2 * 2 * 4);

    let branches = branch_meshes(&tree);
    assert_eq!(branches.len(), 2);
    let b0 = &branches[0];
    assert!(!b0.strip.is_empty());
    assert!(b0.strip.iter().all(|&i| (i as usize) < b0.positions.len()));
    // Every vertex kept is used.
    for i in 0..b0.positions.len() as u16 {
        assert!(b0.strip.contains(&i));
    }
    assert!(!b0.triangles().is_empty());
    // The last level keeps no branches (9008 = 0).
    assert!(branches[1].strip.is_empty());
}

#[test]
fn levels_by_distance() {
    let limits = LodLimits {
        near: 1024.0,
        far: 8192.0,
    };
    let fade = LeafFade {
        mode: 1,
        width: 0.1,
        start: 0.25,
        exponent: 1.5,
    };
    let at = |d: f32| visible(lod_level([0.0; 3], [d, 0.0, 0.0], limits), 2, 2, fade);
    assert_eq!(
        at(100.0).leaves.iter().map(|l| l.0).collect::<Vec<_>>(),
        vec![0]
    );
    // In the first cross-fade both leaf levels show.
    assert_eq!(at(3000.0).leaves.len(), 2);
    // Past the second band only the billboard would: nothing is drawn.
    let far = at(7000.0);
    assert!(far.leaves.is_empty());
}

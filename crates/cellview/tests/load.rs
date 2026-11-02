//! Loads the test room from a temporary Data folder, as the viewer does.

use cellview::{find_data_folder, load, space, Blend, GpuFormat, Options};

fn close(a: [f32; 3], b: [f32; 3]) -> bool {
    a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-3)
}

#[test]
fn loads_a_cell_into_renderer_ready_data() {
    let data = testdata::room("cellview");
    let options = Options {
        official: true,
        ..Options::default()
    };
    let scene = load(data.path(), "TestRoom", &options).unwrap();
    assert_eq!(scene.cell, "TestRoom");

    // Nine models load (the statue's is missing), eleven placements. The
    // lamp that takes its glow from outside is placed twice with different
    // glows, so it has two meshes.
    assert_eq!(scene.meshes.len(), 10);
    assert_eq!(scene.draws.len(), 11);
    // Floor, wall, plank (shared by two models), glow, and the lamps'
    // texture and glow map; the crate's is missing.
    assert_eq!(scene.textures.len(), 6);
    assert!(scene
        .notes
        .iter()
        .any(|n| n.contains("missing model: meshes\\test\\statue.nif")));
    assert!(scene
        .notes
        .iter()
        .any(|n| n.contains("missing texture: textures\\test\\gone.dds")));

    let mesh = |name: &str| {
        scene
            .meshes
            .iter()
            .find(|m| m.name.contains(name))
            .unwrap_or_else(|| panic!("no mesh named {name}"))
    };
    let floor = mesh("floor.nif");
    // Menu callbacks bind exact NIF geometry names, not diagnostic labels.
    assert_eq!(floor.shape_name, "Mesh");
    assert_eq!(floor.material.blend, Blend::Opaque);
    let texture = &scene.textures[floor.material.texture.unwrap()];
    assert_eq!(
        (texture.width, texture.height, texture.mip_levels),
        (8, 8, 1)
    );
    // An uncompressed source goes up as RGBA8 either way.
    assert_eq!(texture.gpu_format(true), GpuFormat::Rgba8);
    let pixels = texture.level_data(GpuFormat::Rgba8).unwrap();
    assert_eq!(&pixels[..4], &[40, 200, 40, 255]);
    assert_eq!(floor.indices.len(), 6);
    assert_eq!(floor.uvs.len(), floor.positions.len());

    // Opaque surfaces test and write depth.
    assert!(floor.material.depth_test && floor.material.depth_write);
    assert_eq!(floor.material.alpha_test, None);

    let glow = mesh("glow.nif");
    assert_eq!(glow.material.blend, Blend::Add);
    assert!(glow.material.unlit && glow.effect);
    assert_eq!(glow.material.color[3], 0.5);
    // Its shader's flags: tested against depth, not writing it.
    assert!(glow.material.depth_test && !glow.material.depth_write);

    // The frame-style shadow keeps its black, half-transparent vertex colors.
    let shadow = mesh("shadow.nif");
    assert_eq!(shadow.material.blend, Blend::Blend);
    // A decal: never writes depth, whatever its flag says; sorted by the
    // centre of its stored bound.
    assert!(shadow.material.decal && !shadow.material.depth_write);
    assert_eq!(shadow.material.sort_center, [0.0; 3]);
    assert!(shadow.material.texture.is_none());
    let colors = shadow.colors.as_ref().unwrap();
    assert!(colors.iter().all(|c| *c == [0.0, 0.0, 0.0, 0.5]));
    assert_eq!(shadow.material.color, [1.0, 1.0, 1.0, 1.0]);

    // The lamp lights itself in white, through its glow map, times the
    // image space's emissive multiplier (2 here).
    let lamp = scene
        .meshes
        .iter()
        .find(|m| m.name.starts_with("meshes\\test\\lamp.nif"))
        .unwrap();
    assert_eq!(lamp.material.emissive, [2.0, 2.0, 2.0]);
    let texture_path = |i: Option<usize>| i.map(|i| scene.textures[i].path.as_str());
    assert_eq!(
        texture_path(lamp.material.texture),
        Some("textures\\test\\lamp.dds")
    );
    assert_eq!(
        texture_path(lamp.material.glow),
        Some("textures\\test\\lamp_g.dds")
    );

    // The other lamp glows in the color of the light its placement names,
    // and in its own white where its placement names nothing (the room has
    // no default weather to take a color from), both doubled.
    let placed_glows: Vec<_> = scene
        .meshes
        .iter()
        .filter(|m| m.name.contains("extlamp.nif"))
        .map(|m| (m.material.emissive, m.material.glow))
        .collect();
    assert_eq!(placed_glows.len(), 2);
    assert!(placed_glows.contains(&([2.0; 3], lamp.material.glow)));
    assert!(placed_glows.contains(&([2.0, 400.0 / 255.0, 200.0 / 255.0], lamp.material.glow)));
    // Surfaces that don't light themselves have no glow map.
    assert_eq!(floor.material.glow, None);

    // A lit mesh whose texture is missing is drawn a neutral grey.
    let crate_ = mesh("crate.nif");
    assert!(crate_.material.texture.is_none());
    assert_eq!(crate_.material.color, [0.6, 0.6, 0.6, 1.0]);

    // The plank turned to face east: its far end (0, 150) lands at (150, 0).
    let plank = scene
        .meshes
        .iter()
        .position(|m| m.name.contains("plank.nif"))
        .unwrap();
    let draw = scene
        .draws
        .iter()
        .find(|d| d.mesh == plank && d.transform[12] == 0.0 && d.transform[13] == 0.0)
        .unwrap();
    assert!(close(
        space::transform_point(&draw.transform, [0.0, 150.0, 0.0]),
        [150.0, 0.0, 0.0]
    ));

    let light = scene.lights[0];
    assert_eq!(scene.lights.len(), 1);
    assert_eq!(light.position, [0.0, 0.0, 5000.0]);
    assert_eq!(light.radius, 300.0);
    assert_eq!(light.fade, 1.5);
    assert!(close(light.color, [1.0, 200.0 / 255.0, 100.0 / 255.0]));
    assert!(close(scene.ambient, [100.0 / 255.0; 3]));
    // The room's directional light is black: none.
    assert_eq!(scene.directional, None);

    // The cell's image space gives the picture its color adjustment.
    let grade = scene.grade.unwrap();
    let c = testdata::IMAGE_SPACE_CINEMATIC;
    assert_eq!(
        (
            grade.saturation,
            grade.contrast_average,
            grade.contrast,
            grade.brightness
        ),
        (c[0], c[1], c[2], c[3])
    );
    assert_eq!((grade.tint, grade.tint_amount), ([c[4], c[5], c[6]], c[7]));
    assert!(scene
        .notes
        .iter()
        .any(|n| n.starts_with("image space TestImageSpace: saturation 0.80")));

    // The camera starts at the coc marker, at eye height, facing east.
    assert!(close(scene.start.eye, [0.0, -150.0, 120.0]));
    assert!((scene.start.heading - std::f32::consts::FRAC_PI_2).abs() < 1e-5);
}

#[test]
fn finds_the_data_folder_from_the_install_folder() {
    let data = testdata::room("cellview-folder");
    let install = std::env::temp_dir().join(format!("nv-rs-install-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&install);
    std::fs::create_dir_all(&install).unwrap();
    std::fs::write(install.join("readme.txt"), b"not the data").unwrap();
    assert_eq!(find_data_folder(data.path()).unwrap(), data.path());
    assert!(find_data_folder(&install).is_err());
    std::fs::remove_dir_all(&install).unwrap();

    let err = load(
        data.path(),
        "Nowhere",
        &Options {
            official: true,
            ..Options::default()
        },
    )
    .err()
    .unwrap();
    assert_eq!(err.to_string(), "no cell matches 'Nowhere'");
}

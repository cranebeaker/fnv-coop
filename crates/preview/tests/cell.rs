//! End to end: a Data folder built in a temp directory (plugin, meshes and
//! textures), loaded, turned into a scene and drawn.

use esm::{ActivePlugins, LoadOrder};
use preview::cell::{build_scene, lighting_for, plan_camera, render, LightingMode, RenderOptions};
use preview::raster::Camera;
use testdata::{close_to, room, FLOOR_RGB, LAMP_GLOW_RGB, LAMP_RGB, PLANK_RGB, WALL_RGB};
use world::RotationConvention;

#[test]
fn draws_a_cell_from_a_data_folder() {
    let data = room("preview");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let assets = assets::Assets::open_with(data.path(), &["FalloutNV.esm".into()], &[]).unwrap();
    let cell_id = world::find_cells(&order, "TestRoom").unwrap()[0];
    let cell = world::load_cell(&order, cell_id).unwrap();
    let scene = build_scene(&assets, cell);

    assert_eq!(scene.instances.len(), 11, "{:?}", scene.report);
    assert_eq!(scene.models.len(), 9);
    let beam = scene
        .models
        .iter()
        .find(|m| m.path.ends_with("beam.nif"))
        .unwrap();
    let turn = beam.root_transform.unwrap();
    assert_eq!(
        preview::cell::describe_transform(&turn),
        "turned 90° anticlockwise"
    );
    let glow = scene
        .models
        .iter()
        .find(|m| m.path.ends_with("glow.nif"))
        .unwrap();
    assert!(glow.meshes[0].effect);
    assert_eq!(glow.meshes[0].opacity, 0.5);
    assert_eq!(
        scene.report.missing_models.keys().collect::<Vec<_>>(),
        ["meshes\\test\\statue.nif"]
    );
    assert_eq!(
        scene.report.missing_textures.keys().collect::<Vec<_>>(),
        ["textures\\test\\gone.dds"]
    );
    let tilted = scene
        .instances
        .iter()
        .filter(|i| scene.is_tilted(i))
        .count();
    assert_eq!(tilted, 1);

    let convention = RotationConvention::DEFAULT;
    let floors = scene.floor_levels(convention);
    assert_eq!(floors[0].0, 0.0);
    let (min, max) = scene.plan_bounds(convention, 160.0).unwrap();
    assert!(min[0] <= -250.0 && max[1] >= 250.0, "{min:?} {max:?}");

    // Plan: 1 pixel = 2 units; the plank runs east from the middle.
    let options = RenderOptions {
        lighting: LightingMode::Cell { brightness: 1.0 },
        supersample: 1,
        ..RenderOptions::default()
    };
    let plan_cam = plan_camera([-256.0, -256.0], [256.0, 256.0], 1000.0, 256, 256);
    // Plans leave light effects out, as render-cell does.
    let plan_options = RenderOptions {
        effects: false,
        ..options
    };
    let plan = render(
        &scene,
        plan_cam,
        256,
        256,
        &plan_options,
        Some(160.0),
        |_, _| {},
    );
    let at = |img: &preview::cell::Rendered, x: f32, y: f32| {
        let (px, py) = plan_cam
            .project([x, y, 0.0], img.width, img.height)
            .unwrap();
        let i = (py as usize * img.width + px as usize) * 4;
        img.rgba[i..i + 3].to_vec()
    };
    // Ambient 100/255 is the only light, so colors are scaled by ~0.39.
    let lit = |rgb: [u8; 3]| rgb.map(|c| (f32::from(c) * 100.0 / 255.0).round() as u8);
    assert!(
        close_to(&at(&plan, 75.0, 0.0), lit(PLANK_RGB), 3),
        "{:?}",
        at(&plan, 75.0, 0.0)
    );
    assert!(close_to(&at(&plan, 0.0, 75.0), lit(FLOOR_RGB), 3));
    assert!(close_to(&at(&plan, -75.0, 0.0), lit(FLOOR_RGB), 3));
    // The beam whose root node is turned lies east, as the game places it.
    assert!(close_to(&at(&plan, 75.0, -200.0), lit(PLANK_RGB), 3));
    assert!(close_to(&at(&plan, 0.0, -125.0), lit(FLOOR_RGB), 3));
    // Keeping the root transform (as viewers do) turns it north instead.
    let viewer_scene =
        preview::cell::build_scene_with(&assets, world::load_cell(&order, cell_id).unwrap(), true);
    let viewer = render(
        &viewer_scene,
        plan_cam,
        256,
        256,
        &plan_options,
        Some(160.0),
        |_, _| {},
    );
    assert!(close_to(&at(&viewer, 0.0, -125.0), lit(PLANK_RGB), 3));
    assert!(close_to(&at(&viewer, 75.0, -200.0), lit(FLOOR_RGB), 3));

    // Counter-clockwise angles would point it west instead.
    let ccw = RenderOptions {
        convention: RotationConvention::parse("xyz-ccw").unwrap(),
        ..options
    };
    let mirrored = render(&scene, plan_cam, 256, 256, &ccw, Some(160.0), |_, _| {});
    assert!(close_to(&at(&mirrored, -75.0, 0.0), lit(PLANK_RGB), 3));
    assert!(close_to(&at(&mirrored, 75.0, 0.0), lit(FLOOR_RGB), 3));
    // The missing texture is drawn grey.
    let grey = at(&plan, -150.0, -150.0);
    assert!(
        grey[0] == grey[1] && grey[1] == grey[2] && grey[0] > 0,
        "{grey:?}"
    );
    // The wall crosses the cut: a white line along the north edge.
    let edge = at(&plan, 0.0, 256.0);
    assert!(edge.iter().all(|&c| c > 150), "{edge:?}");

    // Light effects are left out of the plan...
    assert!(close_to(&at(&plan, 70.0, -70.0), lit(FLOOR_RGB), 3));
    let with_effects = render(&scene, plan_cam, 256, 256, &options, Some(160.0), |_, _| {});
    assert!(at(&with_effects, 70.0, -70.0)[0] > lit(FLOOR_RGB)[0] + 100);
    // ...but drawn in views: added at half strength over the floor.
    let down = Camera::first_person([70.0, -70.0, 300.0], 0.0, -89.9f32.to_radians(), 1.0);
    let view = render(&scene, down, 8, 8, &options, None, |_, _| {});
    let middle = &view.rgba[(4 * 8 + 4) * 4..(4 * 8 + 4) * 4 + 3];
    let expected = lit(FLOOR_RGB).map(|c| c.saturating_add(128));
    assert!(close_to(middle, expected, 3), "{middle:?} vs {expected:?}");

    // The shadow darkens the floor under it by half; it doesn't lighten it.
    let down = Camera::first_person([-70.0, 70.0, 300.0], 0.0, -89.9f32.to_radians(), 1.0);
    let view = render(&scene, down, 8, 8, &options, None, |_, _| {});
    let middle = &view.rgba[(4 * 8 + 4) * 4..(4 * 8 + 4) * 4 + 3];
    let expected = lit(FLOOR_RGB).map(|c| (f32::from(c) * 0.5).round() as u8);
    assert!(close_to(middle, expected, 3), "{middle:?} vs {expected:?}");

    // Self-lit color is added to the light on the lamp's texture, masked by
    // the glow map: only the green channel glows. In the lamp's own white,
    // in the warm light's color where the placement names that light, and
    // in its own white again where the placement names nothing.
    let glowing = |glow: [f32; 3]| {
        [0, 1, 2].map(|k| {
            let light = 100.0 / 255.0 + glow[k] * f32::from(LAMP_GLOW_RGB[k]) / 255.0;
            (f32::from(LAMP_RGB[k]) * light).round() as u8
        })
    };
    let warm = [1.0, 200.0 / 255.0, 100.0 / 255.0];
    for (y, glow) in [(200.0, [1.0; 3]), (120.0, [1.0; 3]), (40.0, warm)] {
        let down = Camera::first_person([200.0, y, 300.0], 0.0, -89.9f32.to_radians(), 1.0);
        let view = render(&scene, down, 8, 8, &options, None, |_, _| {});
        let middle = &view.rgba[(4 * 8 + 4) * 4..(4 * 8 + 4) * 4 + 3];
        let expected = glowing(glow);
        assert!(
            close_to(middle, expected, 3),
            "{y}: {middle:?} vs {expected:?}"
        );
    }

    // View: standing in the middle facing north, the wall fills the centre.
    let camera = Camera::first_person([0.0, -100.0, 120.0], 0.0, 0.0, 75f32.to_radians());
    let view = render(&scene, camera, 64, 36, &options, None, |_, _| {});
    let centre = &view.rgba[(18 * 64 + 32) * 4..(18 * 64 + 32) * 4 + 3];
    assert!(close_to(centre, lit(WALL_RGB), 3), "{centre:?}");
    // From behind, the wall faces away and is culled: nothing but void.
    let back = Camera::first_person(
        [0.0, 320.0, 120.0],
        180f32.to_radians(),
        0.0,
        75f32.to_radians(),
    );
    let view = render(&scene, back, 64, 36, &options, None, |_, _| {});
    let centre = &view.rgba[(18 * 64 + 32) * 4..(18 * 64 + 32) * 4 + 3];
    assert_eq!(centre, [0, 0, 0]);
}

#[test]
fn lit_shaders_use_vertex_colors_whatever_the_flag() {
    let shader = |type_name: &str, shader_flags: u32, shader_flags2: u32| nif::ShaderProperty {
        net: nif::ObjectNet {
            name: String::new(),
            extra_data: Vec::new(),
            controller: -1,
        },
        type_name: type_name.into(),
        lit: type_name.contains("PPLighting"),
        shade_flags: 0,
        shader_type: 0,
        shader_flags,
        shader_flags2,
        env_map_scale: 1.0,
        texture_clamp: 3,
        texture_set: -1,
        file_name: None,
        layout_ok: true,
        falloff: None,
    };
    use preview::cell::vertex_color_use;
    // Doc Mitchell's walls: lit, vertex-color flag (0x20) off, colors used.
    let wall = shader("BSShaderPPLightingProperty", 0x8200_0000, 0x1);
    assert_eq!(vertex_color_use(Some(&wall)), (true, false, false));
    // Vertex alpha still follows its own flag.
    let faded = shader("BSShaderPPLightingProperty", 0x8200_0008, 0x21);
    assert_eq!(vertex_color_use(Some(&faded)), (true, true, false));
    let unlit = shader("BSShaderNoLightingProperty", 0x8200_0000, 0x1);
    assert!(vertex_color_use(Some(&unlit)).0);
}

fn shader_with(type_name: &str, shader_flags: u32, shader_flags2: u32) -> nif::ShaderProperty {
    nif::ShaderProperty {
        net: nif::ObjectNet {
            name: String::new(),
            extra_data: Vec::new(),
            controller: -1,
        },
        type_name: type_name.into(),
        lit: type_name.contains("PPLighting"),
        shade_flags: 0,
        shader_type: 0,
        shader_flags,
        shader_flags2,
        env_map_scale: 1.0,
        texture_clamp: 3,
        texture_set: -1,
        file_name: None,
        layout_ok: true,
        falloff: None,
    }
}

fn mesh_with(
    shader: Option<nif::ShaderProperty>,
    zbuffer: Option<nif::ZBufferProperty>,
    alpha_flags: Option<u16>,
) -> nif::Mesh {
    nif::Mesh {
        name: "Piece".into(),
        block: 0,
        transform: nif::Transform::IDENTITY,
        positions: Vec::new(),
        normals: Vec::new(),
        tangents: Vec::new(),
        bitangents: Vec::new(),
        uvs: Vec::new(),
        colors: Vec::new(),
        triangles: Vec::new(),
        bound: ([0.0; 3], 0.0),
        nodes: Vec::new(),
        billboard: None,
        textures: Vec::new(),
        shader,
        material: None,
        alpha: alpha_flags.map(|flags| nif::AlphaProperty {
            flags,
            threshold: 100,
        }),
        zbuffer,
        double_sided: false,
        skinned: false,
        skin: None,
        property_types: Vec::new(),
    }
}

#[test]
fn depth_test_and_write_follow_the_shaders_flags() {
    use preview::cell::{depth_of, DrawState};
    // The ceiling fan's blades: blended (0x10ED), write flag on: they
    // write depth, as the game's blended windows and bulbs do.
    let fan = mesh_with(
        Some(shader_with("BSShaderPPLightingProperty", 0x8200_0101, 0x1)),
        None,
        Some(0x10ED),
    );
    assert_eq!(depth_of(&fan), (true, true));
    let state = DrawState::of(&fan);
    assert!(state.alpha.blend.is_some() && state.depth_write);
    assert_eq!(
        state.describe(),
        "blended SrcAlpha/InvSrcAlpha, depth test+write"
    );
    // A light beam: write flag off.
    let beam = mesh_with(
        Some(shader_with("BSShaderNoLightingProperty", 0xA200_0148, 0x0)),
        None,
        Some(0x000D),
    );
    assert_eq!(depth_of(&beam), (true, false));
    // A decal (the shadow behind a picture frame): its write flag is on,
    // but the game never writes decals' depth.
    let shadow = mesh_with(
        Some(shader_with("BSShaderNoLightingProperty", 0x8E00_0008, 0x1)),
        None,
        Some(0x00ED),
    );
    assert_eq!(depth_of(&shadow), (true, false));
    assert!(DrawState::of(&shadow).decal);
    // The test flag isn't followed: no scene draw in the recordings had
    // the test off. And an opaque piece writes depth whatever its write
    // flag says: the BOS water crate (`nv_cratebosgeneric01.nif`,
    // 0x82000181 / 0x8000, no write flag) was drawn writing depth at
    // Goodsprings (call 152845241); with the flag followed it was
    // see-through.
    let crate_ = mesh_with(
        Some(shader_with(
            "BSShaderPPLightingProperty",
            0x8200_0181,
            0x8000,
        )),
        None,
        None,
    );
    assert_eq!(depth_of(&crate_), (true, true));
    let untested = mesh_with(
        Some(shader_with("BSShaderPPLightingProperty", 0x0, 0x0)),
        None,
        None,
    );
    assert_eq!(depth_of(&untested), (true, true));
    // Distant-object blocks (flags 0x2000 / 0x4) are tested and written.
    let lod = mesh_with(
        Some(shader_with("BSShaderPPLightingProperty", 0x2000, 0x4)),
        None,
        None,
    );
    assert_eq!(depth_of(&lod), (true, true));
    // Without a shader property: an `NiZBufferProperty`, else both.
    let z = nif::ZBufferProperty { flags: 0x0001 };
    assert_eq!(depth_of(&mesh_with(None, Some(z), None)), (true, false));
    assert_eq!(depth_of(&mesh_with(None, None, None)), (true, true));
    // The decal pull is the recorded D3DRS_DEPTHBIAS.
    assert!((preview::cell::decal_depth_bias() + 2.0e-5).abs() < 1e-11);
}

#[test]
fn dynamic_alpha_pieces_blend_without_an_alpha_property() {
    use preview::cell::alpha_of;
    use preview::raster::BlendFactor;
    // A house's window glow card: dynamic alpha (0x80000), no property.
    let glow = mesh_with(
        Some(shader_with("BSShaderPPLightingProperty", 0x8208_0101, 0x1)),
        None,
        None,
    );
    assert_eq!(
        alpha_of(&glow).blend,
        Some((BlendFactor::SrcAlpha, BlendFactor::InvSrcAlpha))
    );
    // Without the flag: opaque; with a property, the property decides.
    let wall = mesh_with(
        Some(shader_with("BSShaderPPLightingProperty", 0x8200_0101, 0x1)),
        None,
        None,
    );
    assert_eq!(alpha_of(&wall).blend, None);
    let tested = mesh_with(
        Some(shader_with("BSShaderPPLightingProperty", 0x8208_0101, 0x1)),
        None,
        Some(0x12EC),
    );
    assert_eq!(alpha_of(&tested).blend, None);
    assert!(alpha_of(&tested).test.is_some());
}

#[test]
fn shapes_without_a_shader_property_are_not_drawn() {
    use preview::cell::is_drawn;
    // A footlocker's helper quad: only a material.
    let helper = mesh_with(None, None, None);
    assert!(!is_drawn(&helper));
    let lid = mesh_with(
        Some(shader_with("BSShaderPPLightingProperty", 0x8200_0001, 0x1)),
        None,
        None,
    );
    assert!(is_drawn(&lid));
}

#[test]
fn surfaces_that_dont_write_depth_dont_hide_what_comes_after() {
    use preview::raster::{Material, MeshInput, Renderer};
    let camera = Camera::top_down([0.0, 0.0], 100.0, 10.0);
    let quad = |z: f32| {
        (
            vec![
                [-5.0, -5.0, z],
                [5.0, -5.0, z],
                [5.0, 5.0, z],
                [-5.0, 5.0, z],
            ],
            vec![[0u16, 1, 2], [0, 2, 3]],
        )
    };
    let up = [[0.0, 0.0, 1.0]; 4];
    let draw = |r: &mut Renderer, z: f32, color: [f32; 4], depth_write: bool| {
        let (positions, triangles) = quad(z);
        r.draw(
            &MeshInput {
                positions: &positions,
                normals: &up,
                uvs: &[],
                colors: &[],
                triangles: &triangles,
            },
            Material {
                color,
                lit: false,
                depth_write,
                ..Material::default()
            },
        );
    };
    for (depth_write, expected) in [(true, 1.0), (false, 0.0)] {
        let mut r = Renderer::new(8, 8, [0.0; 3], camera);
        // The nearer red one first, then the farther blue one.
        draw(&mut r, 10.0, [1.0, 0.0, 0.0, 1.0], depth_write);
        draw(&mut r, 0.0, [0.0, 0.0, 1.0, 1.0], true);
        r.finish();
        assert_eq!(r.color[4 * 8 + 4][0], expected, "{depth_write}");
    }
}

/// A sequence turning one node about Z, `turns` whole turns over 10 s at
/// an even speed (quadratic keys, as the game's ceiling fan stores it).
fn spin(name: &str, node: &str, turns: f32, looping: bool) -> nif::Sequence {
    let end = -std::f32::consts::TAU * turns;
    let key = |time: f32, value: f32, tangents: (f32, f32)| nif::FloatKey {
        time,
        value,
        tangents: Some(tangents),
        hold: false,
    };
    nif::Sequence {
        name: name.into(),
        start: 0.0,
        stop: 10.0,
        looping,
        tracks: vec![nif::Track {
            node: node.into(),
            priority: 0,
            motion: nif::Motion::Keys {
                translation: Vec::new(),
                rotation: Vec::new(),
                scale: Vec::new(),
                default: (None, None, None),
                euler: Some(Box::new([
                    Vec::new(),
                    Vec::new(),
                    vec![key(0.0, 0.0, (0.0, end)), key(10.0, end, (end, 0.0))],
                ])),
            },
        }],
        accum_root: None,
        materials: Vec::new(),
        text_keys: Vec::new(),
    }
}

#[test]
fn placed_models_play_idle_and_special_idle_else_their_first_sequence() {
    use preview::cell::{opens_and_closes, placed_sequences, Playing};
    let names = |list: Vec<Playing>| {
        list.into_iter()
            .map(|p| {
                if p.runs {
                    p.sequence.name
                } else if p.at_end {
                    format!("{} at its end", p.sequence.name)
                } else {
                    format!("{} held", p.sequence.name)
                }
            })
            .collect::<Vec<_>>()
    };
    let forward = spin("Forward", "Blades", 1.0, false);
    let idle = spin("Idle", "Blades", 10.0, true);
    let special = spin("SpecialIdle", "Light", 1.0, true);
    let fan = [forward.clone(), idle.clone(), special.clone()];
    assert_eq!(
        names(placed_sequences(&fan, false)),
        ["Idle", "SpecialIdle"]
    );
    // Neither: the first sequence, held at its first frame (a mailbox's
    // "Forward" doesn't raise its flag).
    let mailbox = [forward.clone(), spin("Backward", "Blades", 1.0, false)];
    assert_eq!(names(placed_sequences(&mailbox, true)), ["Forward held"]);
    // A footlocker (a container: it opens and closes) is shown closed:
    // "Close" at its end (not played from its start, which would show it
    // open for a moment). Something else with the same model holds
    // "Open"'s first frame.
    let footlocker = [
        spin("Open", "Top", 0.1, false),
        spin("Close", "Top", 0.1, false),
    ];
    let container = opens_and_closes(esm::FourCC::new(b"CONT"));
    assert!(container && !opens_and_closes(esm::FourCC::new(b"STAT")));
    assert_eq!(
        names(placed_sequences(&footlocker, container)),
        ["Close at its end"]
    );
    assert_eq!(names(placed_sequences(&footlocker, false)), ["Open held"]);
    // At its end from the first moment: the lid is where "Close" leaves
    // it, never where it starts.
    let top = vec![("Top".to_string(), nif::Transform::IDENTITY)];
    let all = std::sync::Arc::new(footlocker.to_vec());
    let shut = preview::cell::MeshMotion::of(&top, &placed_sequences(&all, true), &all).unwrap();
    let lid = |seconds: f32| shut.at(seconds).apply_point([100.0, 0.0, 0.0]);
    assert!((lid(0.0)[0] - lid(5.0)[0]).abs() < 1e-3);
    // A tenth of a turn round, from the first moment.
    let turned = 100.0 * (0.1 * std::f32::consts::TAU).cos();
    assert!((lid(0.0)[0] - turned).abs() < 1e-2, "{:?}", lid(0.0));
    // A sequence by name at its own moment: a door's "Open" half way.
    let (seq, at) = shut.sequence_at("open", 0.05).unwrap();
    assert_eq!(seq.name, "Open");
    assert!((at - 0.05).abs() < 1e-6);
    assert_eq!(shut.sequence_at("Open", 20.0).unwrap().1, 10.0);
    assert!(shut.sequence_at("Idle", 0.0).is_none());
    // An `Idle` that doesn't loop isn't played (the game complains).
    let once = spin("Idle", "Blades", 1.0, false);
    assert!(placed_sequences(&[forward, once], false).is_empty());
    assert!(placed_sequences(&[], true).is_empty());
    // Only sequences moving a piece's nodes move it.
    let nodes = vec![("Light".to_string(), nif::Transform::IDENTITY)];
    let all = std::sync::Arc::new(vec![idle.clone(), special]);
    let playing = placed_sequences(&all, false);
    let motion = preview::cell::MeshMotion::of(&nodes, &playing, &all).unwrap();
    assert_eq!(names(motion.sequences), ["SpecialIdle"]);
    let idle_only = std::sync::Arc::new(vec![idle]);
    let playing = placed_sequences(&idle_only, false);
    assert!(preview::cell::MeshMotion::of(&nodes, &playing, &idle_only).is_none());
}

#[test]
fn a_held_first_sequence_stays_at_its_first_frame() {
    // Half a turn over its length, not looping: held, it never turns.
    let nodes = vec![("Flag".to_string(), nif::Transform::IDENTITY)];
    let raise = spin("Forward", "Flag", 0.5, false);
    let all = std::sync::Arc::new(vec![raise.clone()]);
    let held = preview::cell::placed_sequences(&all, false);
    let motion = preview::cell::MeshMotion::of(&nodes, &held, &all).unwrap();
    assert!(!motion.moves());
    let tip = |seconds: f32| motion.at(seconds).apply_point([100.0, 0.0, 0.0]);
    assert!((tip(20.0)[0] - 100.0).abs() < 1e-3, "{:?}", tip(20.0));
    // Run (as an openable's state would), it ends half a turn round.
    let run = preview::cell::Playing {
        sequence: raise,
        runs: true,
        at_end: false,
    };
    let running = preview::cell::MeshMotion::of(&nodes, &[run], &all).unwrap();
    assert!((running.at(20.0).apply_point([100.0, 0.0, 0.0])[0] + 100.0).abs() < 1e-2);
}

#[test]
fn a_ceiling_fans_blades_turn_a_turn_a_second() {
    use nif::Transform;
    // The fan: top node, its `NonAccum` node, the blades' node (the one
    // the sequence turns, resting turned 30° and 20 up), then the shape.
    let turned = |degrees: f32| {
        let (s, c) = degrees.to_radians().sin_cos();
        [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
    };
    let nodes = vec![
        ("fan01".to_string(), Transform::IDENTITY),
        ("fan01 NonAccum".to_string(), Transform::IDENTITY),
        (
            "FanBlades".to_string(),
            Transform {
                rotation: turned(30.0),
                translation: [0.0, 0.0, -20.0],
                scale: 1.0,
            },
        ),
        ("FanBlades:0".to_string(), Transform::IDENTITY),
    ];
    let all = std::sync::Arc::new(vec![spin("Idle", "FanBlades", 10.0, true)]);
    let motion = preview::cell::MeshMotion {
        nodes,
        sequences: preview::cell::placed_sequences(&all, false),
        all: all.clone(),
    };
    // The blade tip resting at (100, 0, -20) in the model (its vertices as
    // loaded): a quarter of a second in, the animation's own angle is −90°
    // (clockwise from above), from its key at 0 (not the node's rest 30°).
    let rest_tip = [100.0, 0.0, -20.0];
    let tip_at = |seconds: f32| motion.at(seconds).apply_point(rest_tip);
    let close = |a: [f32; 3], b: [f32; 3]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-2);
    let expected = |degrees: f32| {
        let (s, c) = degrees.to_radians().sin_cos();
        [100.0 * c, 100.0 * s, -20.0]
    };
    // Rest angle 30°: at t the blades stand at −360° × t, so the tip moves
    // by (−360 t − 30)°.
    assert!(close(tip_at(0.0), expected(-30.0)), "{:?}", tip_at(0.0));
    assert!(close(tip_at(0.25), expected(-120.0)), "{:?}", tip_at(0.25));
    // It loops: 10 s in is 0 s in, and 12.5 s in is half a turn on.
    assert!(close(tip_at(10.0), tip_at(0.0)));
    assert!(close(tip_at(12.5), expected(-210.0)), "{:?}", tip_at(12.5));
}

#[test]
fn glow_cards_under_billboard_nodes_face_the_camera() {
    use nif::Transform;
    use preview::cell::{Billboard, BillboardKind};
    let turned = |degrees: f32| {
        let (s, c) = degrees.to_radians().sin_cos();
        [[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]]
    };
    // A card flat in its node's x–y plane, the node (rigidly facing the
    // camera, mode 2) 40 below the top and turned 30°; the object placed
    // at (100, 0, 0) turned 90°.
    let mut mesh = mesh_with(None, None, None);
    mesh.nodes = vec![
        ("Light".to_string(), Transform::IDENTITY),
        (
            "LightGlow01".to_string(),
            Transform {
                rotation: turned(30.0),
                translation: [0.0, 0.0, -40.0],
                scale: 1.0,
            },
        ),
        ("LightGlow01:0".to_string(), Transform::IDENTITY),
    ];
    mesh.billboard = Some((1, 2));
    let billboard = Billboard::of(&mesh).unwrap();
    assert_eq!(billboard.kind, BillboardKind::FaceCamera);
    // Modes that rotate about the up axis aren't turned.
    mesh.billboard = Some((1, 1));
    assert!(Billboard::of(&mesh).is_none());

    let placement = Transform {
        rotation: turned(90.0),
        translation: [100.0, 0.0, 0.0],
        scale: 2.0,
    };
    // The card's corner (10, 0, -40) and normal as loaded (model space).
    let rest = mesh.nodes[1].1;
    let corner_model = rest.apply_point([10.0, 0.0, 0.0]);
    let close = |a: [f32; 3], b: [f32; 3]| a.iter().zip(b).all(|(x, y)| (x - y).abs() < 1e-3);
    for axes in [
        // Looking north: right is east, up is up, back is south.
        [[1.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0]],
        // Looking straight down: right east, up north, back up.
        [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    ] {
        let facing = billboard.facing(&placement, [0.0; 3], axes);
        let world = |p: [f32; 3]| placement.apply_point(facing.apply_point(p));
        // The node's own point stays put...
        let centre = world(rest.apply_point([0.0; 3]));
        assert!(
            close(centre, placement.apply_point([0.0, 0.0, -40.0])),
            "{centre:?}"
        );
        // ...and its x runs along the picture's right, at its size.
        let along = world(corner_model);
        let expected = [0, 1, 2].map(|k| centre[k] + 20.0 * axes[0][k]);
        assert!(close(along, expected), "{along:?} vs {expected:?}");
    }
}

#[test]
fn the_fov_setting_is_the_width_of_a_4_3_picture() {
    use preview::cell::{horizontal_fov, vertical_fov};
    let degrees = |r: f32| r.to_degrees();
    assert!((degrees(horizontal_fov(75.0, 4.0 / 3.0)) - 75.0).abs() < 1e-3);
    // 1920x1080 in game: 91.3° across, 59.8° up and down.
    assert!((degrees(horizontal_fov(75.0, 16.0 / 9.0)) - 91.31).abs() < 0.01);
    assert!((degrees(vertical_fov(75.0)) - 59.84).abs() < 0.01);
}

#[test]
fn a_dark_ambient_is_used_as_it_is() {
    // Nothing in the game's final pass lifts dark corners, so a dim cell
    // stays dim.
    let data = room("preview-dark");
    let order = LoadOrder::from_data_dir(data.path(), &ActivePlugins::OfficialOnly).unwrap();
    let cell_id = world::find_cells(&order, "TestRoom").unwrap()[0];
    let mut cell = world::load_cell(&order, cell_id).unwrap();
    cell.info.lighting.as_mut().unwrap().ambient = [10, 20, 30];
    let lighting = lighting_for(&cell, LightingMode::Cell { brightness: 1.0 });
    let expected = [10.0, 20.0, 30.0].map(|c: f32| c / 255.0);
    assert_eq!(lighting.ambient, expected);
}

//! A cell's collider: collision markers' primitives and doors' leaves.

use esm::{FormId, FourCC};
use nif::collision::box_shape;
use nif::{CollisionPart, Transform};
use preview::cell::{CellScene, Instance, Model, SceneReport};
use world::{LoadedCell, Placement, Primitive, RotationConvention};

fn placement(id: u32, base: u32, kind: &[u8; 4], position: [f32; 3], heading: f32) -> Placement {
    Placement {
        form_id: FormId(id),
        record_type: FourCC::new(b"REFR"),
        editor_id: None,
        base: FormId(base),
        base_type: FourCC::new(kind),
        base_editor_id: None,
        position,
        rotation: [0.0, 0.0, heading.to_radians()],
        scale: 1.0,
        model: None,
        parts: Vec::new(),
        light: None,
        radius: None,
        teleport: None,
        emittance: None,
        flags: 0,
        enable_parent: None,
        plugin: String::new(),
        actor: None,
        primitive: None,
        open_by_default: false,
    }
}

fn scene(objects: Vec<Placement>, markers: Vec<Placement>, models: Vec<Model>) -> CellScene {
    let mut cell = LoadedCell::actors_only(Vec::new());
    cell.actors.clear();
    let instances = (0..objects.len())
        .map(|i| Instance {
            object: i,
            part: None,
            model: 0,
        })
        .collect();
    cell.objects = objects;
    cell.markers = markers;
    CellScene {
        cell,
        models,
        textures: Vec::new(),
        instances,
        report: SceneReport::default(),
    }
}

fn ray(c: &physics::Collider, from: [f32; 3], to: [f32; 3]) -> Option<f32> {
    c.raycast(from, to, 1000.0).map(|(d, _)| d)
}

#[test]
fn collision_markers_are_solid_and_triggers_are_not() {
    // A plane 400 wide and 300 tall at x = 100, turned to face along x; a
    // box on the non-collidable layer (its `XTRI`); an activator's trigger
    // volume.
    let mut plane = placement(0x902, 0x21, b"STAT", [100.0, 0.0, 0.0], 90.0);
    plane.primitive = Some(Primitive {
        half: [200.0, 1.0, 150.0],
        shape: 3,
        layer: None,
    });
    let mut ghost = placement(0x903, 0x21, b"STAT", [0.0, 500.0, 0.0], 0.0);
    ghost.primitive = Some(Primitive {
        half: [50.0, 50.0, 50.0],
        shape: 1,
        layer: Some(15),
    });
    let mut trigger = placement(0x904, 0x850, b"ACTI", [0.0, -500.0, 0.0], 0.0);
    trigger.primitive = Some(Primitive {
        half: [50.0, 50.0, 50.0],
        shape: 1,
        layer: None,
    });
    let mut ball = placement(0x905, 0x21, b"STAT", [-300.0, 0.0, 0.0], 0.0);
    ball.primitive = Some(Primitive {
        half: [40.0, 0.0, 0.0],
        shape: 2,
        layer: None,
    });
    let scene = scene(Vec::new(), vec![plane, ghost, trigger, ball], Vec::new());
    let mut c = scene.collider(RotationConvention::DEFAULT);
    // The plane, 0.01 deep, stops a ray at x = 100.
    let d = ray(&c, [0.0, 0.0, 50.0], [1.0, 0.0, 0.0]).unwrap();
    assert!((d - 99.99).abs() < 0.01, "{d}");
    // Past its edge (200 to the side), nothing.
    assert!(ray(&c, [0.0, 210.0, 50.0], [1.0, 0.0, 0.0]).is_none());
    // The non-collidable box and the trigger aren't solid.
    assert!(ray(&c, [0.0, 400.0, 0.0], [0.0, 1.0, 0.0]).is_none());
    assert!(ray(&c, [0.0, -400.0, 0.0], [0.0, -1.0, 0.0]).is_none());
    // The sphere, radius 40.
    let d = ray(&c, [0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap();
    assert!((d - 260.0).abs() < 1.0, "{d}");
    // Walking into the plane stops the player its hull's reach (its radius
    // and convex radius, 0.1 Havok units), the box's shell (0.1) and the
    // proxy's keep distance (0.05) short of it.
    c.add(
        &[
            [-1000.0, -1000.0, 0.0],
            [1000.0, -1000.0, 0.0],
            [1000.0, 1000.0, 0.0],
            [-1000.0, 1000.0, 0.0],
        ],
        &[[0, 1, 2], [0, 2, 3]],
    );
    let shape = physics::CharacterShape::PLAYER;
    let mut p = physics::Character::new([0.0, 0.0, 0.0]);
    for _ in 0..120 {
        p.update(&c, &shape, [300.0, 0.0], 1.0 / 60.0);
    }
    let h = nif::collision::HAVOK_SCALE;
    let stop = 100.0 - 0.01 - 0.1 * h - 0.05 * h - (shape.radius + 0.1 * h);
    assert!((p.feet[0] - stop).abs() < 0.1, "{:?} vs {stop}", p.feet);
}

#[test]
fn doors_that_open_own_their_leaves_and_swing_them() {
    // A door model: a fixed frame and a keyframed leaf (the part its
    // animation swings) hanging on a "Leaf" node hinged at x = 50, which
    // "Open" turns a quarter turn anticlockwise over a second.
    let hinge = Transform {
        translation: [50.0, 0.0, 0.0],
        ..Transform::IDENTITY
    };
    let part = |keyframed: bool, centre: [f32; 3]| CollisionPart {
        layer: if keyframed { 2 } else { 1 },
        dynamic: false,
        keyframed,
        node: 0,
        nodes: if keyframed {
            vec![
                ("Door".to_string(), Transform::IDENTITY),
                ("Leaf".to_string(), hinge),
            ]
        } else {
            Vec::new()
        },
        flags: 0,
        shell: 0.0,
        material: 0,
        body: Default::default(),
        shape: box_shape(
            [5.0, 5.0, 100.0],
            &Transform {
                translation: centre,
                ..Transform::IDENTITY
            },
        ),
    };
    // A quaternion (w, x, y, z) for a turn of `t` quarter turns about z.
    let quarter = |t: f32| {
        let (s, c) = (t * std::f32::consts::FRAC_PI_4).sin_cos();
        [c, 0.0, 0.0, s]
    };
    let swing = |name: &str, from: f32, to: f32| nif::Sequence {
        name: name.into(),
        start: 0.0,
        stop: 1.0,
        looping: false,
        tracks: vec![nif::Track {
            node: "Leaf".into(),
            motion: nif::Motion::Keys {
                translation: Vec::new(),
                rotation: vec![(0.0, quarter(from)), (1.0, quarter(to))],
                scale: Vec::new(),
                default: (None, None, None),
                euler: None,
            },
            priority: 0,
        }],
        accum_root: None,
        materials: Vec::new(),
        text_keys: Vec::new(),
    };
    let sequences = std::sync::Arc::new(vec![swing("Open", 0.0, 1.0), swing("Close", 1.0, 0.0)]);
    let model = Model {
        path: "meshes\\test\\door.nif".into(),
        meshes: Vec::new(),
        collision: vec![
            part(false, [-60.0, 0.0, 100.0]),
            part(true, [0.0, 0.0, 100.0]),
        ],
        root_transform: None,
        skeleton: None,
        sequences,
        particles: None,
        bsx_flags: 0,
    };
    let closed = placement(0x904, 0x830, b"DOOR", [0.0, 0.0, 0.0], 0.0);
    let mut open = placement(0x905, 0x830, b"DOOR", [0.0, 300.0, 0.0], 0.0);
    open.open_by_default = true;
    let mut load = placement(0x906, 0x830, b"DOOR", [0.0, 600.0, 0.0], 0.0);
    load.open_by_default = true;
    load.teleport = Some(world::Teleport {
        door: FormId(0x907),
        position: [0.0; 3],
        rotation: [0.0; 3],
    });
    let scene = scene(vec![closed, open, load], Vec::new(), vec![model]);
    let mut c = scene.collider(RotationConvention::DEFAULT);
    // Rays from 50 units short of each door, 100 units long.
    let near = |c: &physics::Collider, from: [f32; 3]| c.raycast(from, [0.0, 1.0, 0.0], 100.0);
    let leaf = |c: &physics::Collider, y: f32| near(c, [0.0, y - 50.0, 100.0]);
    let frame = |c: &physics::Collider, y: f32| near(c, [-60.0, y - 50.0, 100.0]);
    // Every door's leaf is added where the model files it (closed); the
    // leaves of the two that open belong to them, the load door's to
    // nobody (it never opens).
    assert!(leaf(&c, 0.0).is_some() && frame(&c, 0.0).is_some());
    assert!(leaf(&c, 300.0).is_some() && frame(&c, 300.0).is_some());
    assert!(c.owns(0x904) && c.owns(0x905));
    assert!(leaf(&c, 600.0).is_some() && !c.owns(0x906));
    // Whatever owns them, every part's triangles name the reference they
    // come from, frame and leaf, for the crosshair's pick
    // (`physics::view_caster`): the load door's too.
    for (y, door) in [(0.0, 0x904), (300.0, 0x905), (600.0, 0x906)] {
        for (_, t) in [leaf(&c, y), frame(&c, y)].into_iter().flatten() {
            assert_eq!(c.reference(t), door);
        }
        assert!(c
            .raycast_reference([-60.0, y - 50.0, 100.0], [0.0, 1.0, 0.0], door)
            .is_some());
    }
    // The doors that swing, with their model's sequences and the leaf's
    // node chain.
    let doors = scene.swing_doors(RotationConvention::DEFAULT);
    assert_eq!(doors.len(), 2);
    let door = &doors[0];
    assert_eq!(door.reference, FormId(0x904));
    assert!(!door.open_by_default && doors[1].open_by_default);
    assert_eq!(
        door.lengths(),
        Some(world::doors::Lengths {
            open: 1.0,
            close: 1.0
        })
    );
    assert_eq!(door.leaves.len(), 1);
    // "Open" at its end turns the leaf a quarter turn about the hinge:
    // the collider's leaf swings with it, out of the ray's way and into
    // a ray from the hinge's side.
    let (seq, at) = door.sequence_at(true, 1.0).unwrap();
    assert_eq!(seq.name, "Open");
    let (rotation, translation) = door.leaf_move(Some((seq, at))).unwrap();
    c.move_owner(0x904, &rotation, translation);
    assert!(leaf(&c, 0.0).is_none(), "the leaf still stands across");
    // The leaf's middle (0,0) went a quarter turn about the hinge at
    // (50,0), to (50,±50): a ray along +x at that y meets it 45 units in.
    let across = |y: f32| {
        c.raycast([0.0, y, 100.0], [1.0, 0.0, 0.0], 200.0)
            .map(|(d, _)| d)
    };
    let hit = across(-50.0).or_else(|| across(50.0));
    assert!(hit.is_some_and(|d| (d - 45.0).abs() < 1e-2), "{hit:?}");
    // Back to its start: solid across again.
    let (seq, at) = door.sequence_at(false, 1.0).unwrap();
    let (rotation, translation) = door.leaf_move(Some((seq, at))).unwrap();
    c.move_owner(0x904, &rotation, translation);
    assert!(leaf(&c, 0.0).is_some());
    // The frame and the other doors never moved.
    assert!(frame(&c, 0.0).is_some() && leaf(&c, 300.0).is_some());
}

#[test]
fn a_placed_scale_scales_the_collision() {
    // The game clones a scaled reference's bodies scaled (`00c8f2a0`).
    let model = Model {
        path: "meshes\\test\\crate.nif".into(),
        meshes: Vec::new(),
        collision: vec![CollisionPart {
            layer: 1,
            dynamic: false,
            keyframed: false,
            node: 0,
            nodes: Vec::new(),
            flags: 0,
            shell: 0.0,
            material: 0,
            body: Default::default(),
            shape: box_shape([10.0, 10.0, 10.0], &Transform::IDENTITY),
        }],
        root_transform: None,
        skeleton: None,
        sequences: Default::default(),
        particles: None,
        bsx_flags: 0,
    };
    let mut crate_ = placement(0x908, 0x803, b"STAT", [0.0, 0.0, 0.0], 0.0);
    crate_.scale = 2.0;
    let scene = scene(vec![crate_], Vec::new(), vec![model]);
    let c = scene.collider(RotationConvention::DEFAULT);
    let d = ray(&c, [-100.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap();
    assert!((d - 80.0).abs() < 1e-3, "{d}");
}

#[test]
fn moving_clutter_is_a_body_and_left_out_of_the_collider() {
    // A bottle-like prop (layer 10, motion 4, mass 1) and a static shelf
    // under it (friction 0.8), as models; both placed, the bottle at scale 2.
    let body = |block: usize, mass: f32, motion: u8, friction: f32| nif::RigidBodyInfo {
        block,
        mass,
        center: [0.0, 0.0, 5.0],
        inertia: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 0.5]],
        friction,
        restitution: 0.4,
        motion,
        ..Default::default()
    };
    let part = |layer: u8, dynamic: bool, info: nif::RigidBodyInfo, half: [f32; 3]| CollisionPart {
        layer,
        dynamic,
        keyframed: false,
        node: 0,
        nodes: Vec::new(),
        flags: 0,
        shell: 0.7,
        material: 0,
        body: info,
        shape: box_shape(half, &Transform::IDENTITY),
    };
    let model = |collision: Vec<CollisionPart>| Model {
        path: "meshes\\test\\thing.nif".into(),
        meshes: Vec::new(),
        collision,
        root_transform: None,
        skeleton: None,
        sequences: Default::default(),
        particles: None,
        bsx_flags: 0,
    };
    let bottle = model(vec![part(10, true, body(4, 1.0, 4, 0.5), [2.0, 2.0, 10.0])]);
    let shelf = model(vec![part(
        1,
        false,
        body(3, 0.0, 7, 0.8),
        [50.0, 50.0, 5.0],
    )]);
    let mut b = placement(0x910, 0x804, b"MISC", [0.0, 0.0, 100.0], 90.0);
    b.scale = 2.0;
    let s = placement(0x911, 0x805, b"STAT", [0.0, 0.0, 0.0], 0.0);
    let mut scene = scene(vec![b, s], Vec::new(), vec![bottle, shelf]);
    scene.instances[1].model = 1;
    let bodies = scene.dynamic_bodies(RotationConvention::DEFAULT);
    assert_eq!(bodies.len(), 1);
    let d = &bodies[0];
    assert_eq!((d.reference, d.setup.reference), (FormId(0x910), 0x910));
    assert_eq!((d.setup.layer, d.setup.mass, d.setup.motion), (10, 1.0, 4));
    // The scale in the shapes, the centre and the inertia.
    assert_eq!(d.setup.center, [0.0, 0.0, 10.0]);
    assert_eq!(d.setup.inertia[2][2], 2.0);
    let physics::rigid::Shape::Hull { vertices, .. } = &d.setup.shapes[0] else {
        panic!("{:?}", d.setup.shapes[0])
    };
    let top = vertices.iter().map(|v| v[2]).fold(f32::MIN, f32::max);
    assert!((top - 20.0).abs() < 1e-4, "{top}");
    // Placed turned 90° at its position.
    assert_eq!(d.pose.1, [0.0, 0.0, 100.0]);
    assert!((d.pose.0[1][0].abs() - 1.0).abs() < 1e-5, "{:?}", d.pose.0);
    // The collider has the shelf (with its friction) but not the bottle.
    let c = scene.collider(RotationConvention::DEFAULT);
    let (hit, tri) = c
        .raycast([0.0, 0.0, 200.0], [0.0, 0.0, -1.0], 500.0)
        .unwrap();
    assert!((hit - 195.0).abs() < 1e-3, "{hit}");
    assert_eq!(c.surface(tri).map(|s| s.friction), Some(0.8));
    assert!(!c.owns(0x910));
}

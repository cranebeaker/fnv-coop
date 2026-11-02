//! Particle system models (`nif::particles`, `world::particles`): a dust
//! cloud like the game's `fxlightdustparticleswide02.nif` (a cylinder
//! emitter, a three-colour fade, rotation), with optional gravity, a
//! planar collider, an atlas, additive blending, and its emitter driven by
//! an `Idle` sequence instead of its own interpolators.

use crate::{f32s, sized, NifBuilder};

/// What the dust model has.
#[derive(Debug, Clone)]
pub struct Dust {
    /// Births a second, life span (no variation), the most particles.
    pub rate: f32,
    pub life: f32,
    pub max: u16,
    pub world_space: bool,
    /// The emitter controller's key range, phase and "emitter active"
    /// keys.
    pub range: (f32, f32),
    pub phase: f32,
    pub active: Vec<(f32, bool)>,
    /// The emitter controller gets its interpolators from an `Idle`
    /// sequence (looping over `range`) instead of its own.
    pub sequence: bool,
    /// Gravity of this strength along −z (planar, the root as its object).
    pub gravity: Option<f32>,
    /// A floor collider at the model's origin (facing up) with this
    /// bounce.
    pub floor: Option<f32>,
    /// Blending `SrcAlpha`/`One` (else `SrcAlpha`/`InvSrcAlpha`).
    pub additive: bool,
    /// The material's alpha.
    pub material_alpha: f32,
    /// A 2 × 2 atlas.
    pub atlas: bool,
    /// Speed (and its variation) along the emitter's declination.
    pub speed: (f32, f32),
    /// Declination and its variation (radians from +z).
    pub declination: (f32, f32),
    /// With `sequence`, the `Idle` sequence also moves the emitter's node
    /// along a path (`NiTransformController` + `NiPathInterpolator`, as the
    /// dust whirlwind's mover).
    pub path_mover: bool,
}

impl Default for Dust {
    /// The game's barracks dust: 7.5 a second living 8 s, 60 at most, in
    /// world space, on from 0 to 8 and looping.
    fn default() -> Self {
        Dust {
            rate: 7.5,
            life: 8.0,
            max: 60,
            world_space: true,
            range: (0.0, 8.0),
            phase: 0.0,
            active: vec![(0.0, true), (8.0, false)],
            sequence: false,
            gravity: None,
            floor: None,
            additive: false,
            material_alpha: 1.0,
            atlas: false,
            speed: (7.5, 12.75),
            declination: (std::f32::consts::PI, 2.042_035),
            path_mover: false,
        }
    }
}

/// The emitter's name, as controllers and sequences name it.
pub const EMITTER: &str = "NiPSysCylinderEmitter:0";
/// The system's name.
pub const SYSTEM: &str = "Dust";

/// The dust model.
pub fn dust_nif(d: &Dust) -> Vec<u8> {
    let mut b = NifBuilder::default();
    // Blocks, in order.
    const ROOT: i32 = 0;
    const EMITTER_NODE: i32 = 1;
    const SYSTEM_BLOCK: i32 = 2;
    const SHADER: i32 = 3;
    const ALPHA: i32 = 4;
    const MATERIAL: i32 = 5;
    const DATA: i32 = 6;
    const AGE_DEATH: i32 = 7;
    const EMITTER_BLOCK: i32 = 8;
    const COLOR: i32 = 9;
    const ROTATION: i32 = 10;
    const GRAVITY: i32 = 11;
    const COLLIDERS: i32 = 12;
    const POSITION: i32 = 13;
    const EMITTER_CTLR: i32 = 14;
    const UPDATE_CTLR: i32 = 15;
    const RATE: i32 = 16;
    const ACTIVE: i32 = 17;
    const ACTIVE_DATA: i32 = 18;
    const PLANE: i32 = 19;
    const SEQUENCE: i32 = 20;

    let mut root = b.av("Root", &[]);
    root.extend(2u32.to_le_bytes());
    root.extend(EMITTER_NODE.to_le_bytes());
    root.extend(SYSTEM_BLOCK.to_le_bytes());
    root.extend(0u32.to_le_bytes());

    let mut emitter_node = b.av("Emitter", &[]);
    emitter_node.extend(0u32.to_le_bytes());
    emitter_node.extend(0u32.to_le_bytes());

    let mut system = b.av(SYSTEM, &[SHADER, ALPHA, MATERIAL]);
    system[8..12].copy_from_slice(&EMITTER_CTLR.to_le_bytes());
    system.extend(DATA.to_le_bytes());
    system.extend((-1i32).to_le_bytes()); // skin
    system.extend(0u32.to_le_bytes()); // materials
    system.extend((-1i32).to_le_bytes()); // active material
    system.push(0); // needs update
    system.push(u8::from(d.world_space));
    let mut modifiers = vec![AGE_DEATH, EMITTER_BLOCK, COLOR, ROTATION];
    if d.gravity.is_some() {
        modifiers.push(GRAVITY);
    }
    if d.floor.is_some() {
        modifiers.push(COLLIDERS);
    }
    modifiers.push(POSITION);
    system.extend((modifiers.len() as u32).to_le_bytes());
    for m in &modifiers {
        system.extend(m.to_le_bytes());
    }

    // BSShaderNoLightingProperty: flags as the game's dust (external
    // emittance), no depth write.
    let mut shader = b.net("");
    shader.extend(1u16.to_le_bytes());
    for v in [1u32, 0xA200_0108, 0] {
        shader.extend(v.to_le_bytes());
    }
    shader.extend(1.0f32.to_le_bytes());
    shader.extend(3u32.to_le_bytes());
    shader.extend(sized("textures\\effects\\dust.dds"));
    shader.extend(f32s(&[1.0, 0.0, 1.0, 0.0]));

    let mut alpha = b.net("");
    alpha.extend((if d.additive { 0x120Du16 } else { 0x12EDu16 }).to_le_bytes());
    alpha.push(1);

    let mut material = b.net("");
    material.extend(f32s(&[
        0.0,
        0.0,
        0.0,
        1.0,
        1.0,
        1.0,
        10.0,
        d.material_alpha,
        1.0,
    ]));

    // NiPSysData: no arrays stored, only which exist.
    let mut data = 0i32.to_le_bytes().to_vec();
    data.extend(d.max.to_le_bytes());
    data.extend([0, 0, 0]); // keep, compress, has vertices
    data.extend(0u16.to_le_bytes()); // data flags
    data.push(0); // has normals
    data.extend(f32s(&[0.0, 0.0, 0.0, 0.0])); // bound
    data.push(1); // has colours
    data.extend(0u16.to_le_bytes()); // consistency
    data.extend((-1i32).to_le_bytes()); // additional data
    data.push(1); // has radii
    data.extend(0u16.to_le_bytes()); // active count
    data.extend([1, 0, 1, 0, u8::from(d.atlas)]); // sizes, rotations, angles, axes, indices
    let atlas: Vec<[f32; 4]> = if d.atlas {
        vec![
            [0.0, 0.5, 0.0, 0.5],
            [0.5, 0.5, 0.0, 0.5],
            [0.0, 0.5, 0.5, 0.5],
            [0.5, 0.5, 0.5, 0.5],
        ]
    } else {
        Vec::new()
    };
    data.push(atlas.len() as u8);
    for piece in &atlas {
        data.extend(f32s(piece));
    }
    data.push(1); // rotation speeds

    let modifier = |b: &mut NifBuilder, name: &str, order: u32| {
        let mut m = b.string(name).to_le_bytes().to_vec();
        m.extend(order.to_le_bytes());
        m.extend(SYSTEM_BLOCK.to_le_bytes());
        m.push(1);
        m
    };
    let mut age_death = modifier(&mut b, "NiPSysAgeDeath:2", 0);
    age_death.push(0);
    age_death.extend((-1i32).to_le_bytes());

    let mut emitter = modifier(&mut b, EMITTER, 1000);
    emitter.extend(f32s(&[
        d.speed.0,
        d.speed.1,
        d.declination.0,
        d.declination.1,
        0.0,
        4.084_07,
        1.0,
        1.0,
        1.0,
        1.0,
        2.0,
        0.5,
        d.life,
        0.0,
    ]));
    emitter.extend(EMITTER_NODE.to_le_bytes());
    emitter.extend(f32s(&[256.0, 256.0]));

    // The dust's three colours: in over 35% of the life, out after 65%.
    let mut color = modifier(&mut b, "BSPSysSimpleColorModifier:3", 3000);
    color.extend(f32s(&[0.35, 0.65, 0.0, 0.0, 0.0, 0.0]));
    color.extend(f32s(&[0.0, 0.0, 0.0, 0.0]));
    color.extend(f32s(&[0.619_607_9, 0.650_980_4, 0.670_588_3, 0.45]));
    color.extend(f32s(&[0.0, 0.0, 0.0, 0.0]));

    let mut rotation = modifier(&mut b, "NiPSysRotationModifier:4", 3000);
    rotation.extend(f32s(&[
        0.139_626_34,
        0.122_173_05,
        -std::f32::consts::PI,
        std::f32::consts::PI,
    ]));
    rotation.extend([1, 1]);
    rotation.extend(f32s(&[1.0, 0.0, 0.0]));

    let mut gravity = modifier(&mut b, "NiPSysGravityModifier:5", 4000);
    gravity.extend(ROOT.to_le_bytes());
    gravity.extend(f32s(&[0.0, 0.0, -1.0, 0.0, d.gravity.unwrap_or(0.0)]));
    gravity.extend(0u32.to_le_bytes());
    gravity.extend(f32s(&[0.0, 0.0]));
    gravity.push(0);

    let mut colliders = modifier(&mut b, "NiPSysColliderManager:6", 5000);
    colliders.extend(PLANE.to_le_bytes());

    let position = modifier(&mut b, "NiPSysPositionModifier:7", 6000);

    let controller = |next: i32, flags: u16| {
        let mut c = next.to_le_bytes().to_vec();
        c.extend(flags.to_le_bytes());
        c.extend(f32s(&[1.0, d.phase, d.range.0, d.range.1]));
        c.extend(SYSTEM_BLOCK.to_le_bytes());
        c
    };
    // Active, looping; under a controller manager with a sequence.
    let mut emitter_ctlr = controller(UPDATE_CTLR, if d.sequence { 0x0068 } else { 0x0048 });
    emitter_ctlr.extend((if d.sequence { -1 } else { RATE }).to_le_bytes());
    emitter_ctlr.extend(b.string(EMITTER).to_le_bytes());
    emitter_ctlr.extend((if d.sequence { -1 } else { ACTIVE }).to_le_bytes());
    let update_ctlr = controller(-1, 0x004C);

    let mut rate = f32s(&[d.rate]);
    rate.extend((-1i32).to_le_bytes());
    let mut active = vec![2u8];
    active.extend(ACTIVE_DATA.to_le_bytes());
    let mut active_data = (d.active.len() as u32).to_le_bytes().to_vec();
    if !d.active.is_empty() {
        active_data.extend(5u32.to_le_bytes());
    }
    for (t, on) in &d.active {
        active_data.extend(t.to_le_bytes());
        active_data.push(u8::from(*on));
    }

    let mut plane = f32s(&[d.floor.unwrap_or(0.0)]);
    plane.extend([0, 0]);
    plane.extend((-1i32).to_le_bytes()); // spawn modifier
    plane.extend(COLLIDERS.to_le_bytes()); // manager
    plane.extend((-1i32).to_le_bytes()); // next
    plane.extend(ROOT.to_le_bytes()); // object (the plane at the root, moved below)
    plane.extend(f32s(&[10_000.0, 10_000.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]));

    let mut blocks: Vec<(String, Vec<u8>)> = vec![
        ("BSFadeNode".into(), root),
        ("NiNode".into(), emitter_node),
        ("NiParticleSystem".into(), system),
        ("BSShaderNoLightingProperty".into(), shader),
        ("NiAlphaProperty".into(), alpha),
        ("NiMaterialProperty".into(), material),
        ("NiPSysData".into(), data),
        ("NiPSysAgeDeathModifier".into(), age_death),
        ("NiPSysCylinderEmitter".into(), emitter),
        ("BSPSysSimpleColorModifier".into(), color),
        ("NiPSysRotationModifier".into(), rotation),
        ("NiPSysGravityModifier".into(), gravity),
        ("NiPSysColliderManager".into(), colliders),
        ("NiPSysPositionModifier".into(), position),
        ("NiPSysEmitterCtlr".into(), emitter_ctlr),
        ("NiPSysUpdateCtlr".into(), update_ctlr),
        ("NiFloatInterpolator".into(), rate),
        ("NiBoolInterpolator".into(), active),
        ("NiBoolData".into(), active_data),
        ("NiPSysPlanarCollider".into(), plane),
    ];
    if d.sequence {
        let path = SEQUENCE + 1;
        let mut s = b.string("Idle").to_le_bytes().to_vec();
        s.extend((if d.path_mover { 3u32 } else { 2u32 }).to_le_bytes());
        s.extend(1u32.to_le_bytes());
        for (interp, id) in [(RATE, "BirthRate"), (ACTIVE, "EmitterActive")] {
            s.extend(interp.to_le_bytes());
            s.extend(EMITTER_CTLR.to_le_bytes());
            s.push(0);
            s.extend(b.string(SYSTEM).to_le_bytes());
            s.extend((-1i32).to_le_bytes());
            s.extend(b.string("NiPSysEmitterCtlr").to_le_bytes());
            s.extend(b.string(EMITTER).to_le_bytes());
            s.extend(b.string(id).to_le_bytes());
        }
        if d.path_mover {
            s.extend(path.to_le_bytes());
            s.extend((-1i32).to_le_bytes());
            s.push(0);
            s.extend(b.string("Emitter").to_le_bytes());
            s.extend((-1i32).to_le_bytes());
            s.extend(b.string("NiTransformController").to_le_bytes());
            s.extend((-1i32).to_le_bytes());
            s.extend((-1i32).to_le_bytes());
        }
        s.extend(1.0f32.to_le_bytes()); // weight
        s.extend((-1i32).to_le_bytes()); // text keys
        s.extend(0u32.to_le_bytes()); // loop
        s.extend(f32s(&[1.0, d.range.0, d.range.1]));
        s.extend((-1i32).to_le_bytes()); // manager
        s.extend((-1i32).to_le_bytes()); // accumulation root
        blocks.push(("NiControllerSequence".into(), s));
        if d.path_mover {
            // Its contents aren't read (only its type).
            blocks.push(("NiPathInterpolator".into(), vec![0; 4]));
        }
    }
    b.blocks = blocks;
    b.build()
}

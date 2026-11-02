//! Particle blocks read from models built from scratch (`testdata::particles`),
//! laid out as the game's loaders read them.

use nif::particles::{
    ColliderShape, ControllerKind, EmitterShape, ModifierKind, ParticleBlock, TrackValue,
};
use nif::Nif;
use testdata::particles::{dust_nif, Dust, EMITTER, SYSTEM};

#[test]
fn every_particle_block_is_read_exactly() {
    let dust = Dust {
        gravity: Some(10.0),
        floor: Some(0.5),
        atlas: true,
        sequence: true,
        ..Dust::default()
    };
    let nif = Nif::parse(dust_nif(&dust)).unwrap();
    let mut read = 0;
    for i in 0..nif.blocks().len() {
        let t = nif.block_type(i).to_string();
        if nif::particles::is_particle_type(&t)
            || matches!(
                t.as_str(),
                "NiFloatInterpolator" | "NiBoolInterpolator" | "NiPSysPlanarCollider"
            )
        {
            nif.particle_block(i).unwrap_or_else(|e| panic!("{t}: {e}"));
            read += 1;
        }
    }
    // The system, its data, seven modifiers, two controllers, two
    // interpolators and the collider.
    assert_eq!(read, 14);
    assert!(matches!(
        nif.particle_block(2).unwrap(),
        ParticleBlock::System {
            world_space: true,
            ..
        }
    ));
}

#[test]
fn a_dust_cloud_reads_like_the_games() {
    let nif = Nif::parse(dust_nif(&Dust {
        gravity: Some(10.0),
        floor: Some(0.5),
        ..Dust::default()
    }))
    .unwrap();
    let systems = nif.particle_systems(false).unwrap();
    assert_eq!(systems.len(), 1);
    let s = &systems[0];
    assert_eq!(s.name, SYSTEM);
    assert!(s.world_space);
    assert_eq!(s.data.max_particles, 60);
    assert!(s.data.has_colors && s.data.has_rotation_angles && s.data.has_rotation_speeds);
    assert_eq!(s.texture.as_deref(), Some("textures\\effects\\dust.dds"));
    assert_eq!(s.alpha.map(|a| a.flags), Some(0x12ED));
    // The modifiers in the list's order.
    let kinds: Vec<&str> = s
        .modifiers
        .iter()
        .map(|m| match &m.kind {
            ModifierKind::AgeDeath { .. } => "age",
            ModifierKind::Emitter(_) => "emitter",
            ModifierKind::SimpleColor(_) => "colour",
            ModifierKind::Rotation(_) => "rotation",
            ModifierKind::Gravity(_) => "gravity",
            ModifierKind::Colliders(_) => "colliders",
            ModifierKind::Position => "position",
            _ => "other",
        })
        .collect();
    assert_eq!(
        kinds,
        [
            "age",
            "emitter",
            "colour",
            "rotation",
            "gravity",
            "colliders",
            "position"
        ]
    );
    let ModifierKind::Emitter(e) = &s.modifiers[1].kind else {
        panic!("no emitter");
    };
    assert_eq!((e.speed, e.life_span, e.radius), (7.5, 8.0, 2.0));
    let EmitterShape::Cylinder {
        object: Some(o),
        radius,
        height,
    } = &e.shape
    else {
        panic!("{:?}", e.shape);
    };
    assert_eq!(
        (o.name.as_str(), *radius, *height),
        ("Emitter", 256.0, 256.0)
    );
    let ModifierKind::SimpleColor(c) = &s.modifiers[2].kind else {
        panic!("no colours");
    };
    assert_eq!((c.fade_in, c.fade_out, c.colors[1][3]), (0.35, 0.65, 0.45));
    let ModifierKind::Colliders(colliders) = &s.modifiers[5].kind else {
        panic!("no colliders");
    };
    assert_eq!(colliders[0].bounce, 0.5);
    assert!(matches!(
        colliders[0].shape,
        ColliderShape::Plane { width, .. } if width == 10_000.0
    ));
    // The controller chain: the emitter's, then the update.
    assert_eq!(s.controllers.len(), 2);
    let ControllerKind::Emitter {
        modifier,
        birth_rate: Some(rate),
        active: Some(active),
        ..
    } = &s.controllers[0].kind
    else {
        panic!("{:?}", s.controllers[0].kind);
    };
    assert_eq!(modifier, EMITTER);
    assert_eq!(rate.value, 7.5);
    assert_eq!(active.keys, vec![(0.0, true), (8.0, false)]);
    // The loader sets "compute scaled time" on every controller.
    assert_eq!(s.controllers[0].time.flags, 0x48);
    assert_eq!(s.controllers[1].kind, ControllerKind::Update);
}

#[test]
fn sequences_name_the_nodes_they_move_along_paths() {
    let nif = Nif::parse(dust_nif(&Dust {
        sequence: true,
        path_mover: true,
        ..Dust::default()
    }))
    .unwrap();
    let sequences = nif.particle_sequences().unwrap();
    assert_eq!(sequences[0].unread_movers, vec!["Emitter".to_string()]);
    assert_eq!(sequences[0].tracks.len(), 2);
    let plain = Nif::parse(dust_nif(&Dust {
        sequence: true,
        ..Dust::default()
    }))
    .unwrap();
    assert!(plain.particle_sequences().unwrap()[0]
        .unread_movers
        .is_empty());
}

#[test]
fn sequences_hand_particle_controllers_their_interpolators() {
    let nif = Nif::parse(dust_nif(&Dust {
        sequence: true,
        ..Dust::default()
    }))
    .unwrap();
    let systems = nif.particle_systems(false).unwrap();
    assert!(systems[0].controllers[0].time.manager_controlled());
    let sequences = nif.particle_sequences().unwrap();
    assert_eq!(sequences.len(), 1);
    let s = &sequences[0];
    assert_eq!(
        (s.name.as_str(), s.cycle, s.start, s.stop),
        ("Idle", 0, 0.0, 8.0)
    );
    assert_eq!(s.tracks.len(), 2);
    assert!(s.tracks.iter().all(|t| t.node == SYSTEM
        && t.controller_type == "NiPSysEmitterCtlr"
        && t.controller_id == EMITTER));
    assert!(matches!(&s.tracks[0].value, TrackValue::Float(f) if f.value == 7.5));
    assert!(matches!(&s.tracks[1].value, TrackValue::Bool(b) if b.keys.len() == 2 && !b.timeline));
}

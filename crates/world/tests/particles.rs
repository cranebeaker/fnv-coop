//! Particle systems run as the game runs them (`world::particles`), on
//! models built from scratch (`testdata::particles`).

use std::sync::Arc;

use nif::particles::{ParticleSequence, ParticleSystem, TimeControl};
use nif::{Nif, Transform};
use testdata::particles::{dust_nif, Dust};
use world::particles::{
    bool_at, compute_scaled_time, emission_ages, pack_color, sequence_time, CameraBasis, Clock,
    PlacedParticles, Rand, Range, SinCos,
};

fn model(d: &Dust) -> (Vec<Arc<ParticleSystem>>, Vec<ParticleSequence>) {
    let nif = Nif::parse(dust_nif(d)).unwrap();
    let systems = nif
        .particle_systems(false)
        .unwrap()
        .into_iter()
        .map(Arc::new)
        .collect();
    (systems, nif.particle_sequences().unwrap())
}

/// Runs a model at 60 frames a second for `seconds`, playing its first
/// sequence when `play`.
fn run(d: &Dust, seconds: f32, play: bool) -> PlacedParticles {
    let (systems, sequences) = model(d);
    let playing = if play && !sequences.is_empty() {
        vec![(0, true)]
    } else {
        Vec::new()
    };
    let mut placed = PlacedParticles::new(&systems, sequences, playing, 7);
    let frames = (seconds * 60.0).round() as usize;
    for frame in 0..=frames {
        placed.update(frame as f32 / 60.0, &Transform::IDENTITY, [0.0; 3]);
    }
    placed
}

#[test]
fn random_numbers_are_the_c_runtimes() {
    // Microsoft's rand() from seed 1.
    let mut r = Rand(1);
    let first: Vec<i32> = (0..5).map(|_| r.draw()).collect();
    assert_eq!(first, [41, 18467, 6334, 26500, 19169]);
    let mut r = Rand(1);
    assert_eq!(r.unit(), (41.0f64 / 32767.0) as f32);
}

#[test]
fn angles_are_looked_up_in_512_step_tables() {
    let t = SinCos::new();
    assert_eq!(t.sin[0], 0.0);
    assert!((t.sin[128] - 1.0).abs() < 1e-6);
    assert!((t.cos[256] + 1.0).abs() < 1e-6);
    assert_eq!(SinCos::index(std::f32::consts::FRAC_PI_2), 128);
    // Truncated toward zero, then masked: −8.1 → −8 → 504.
    assert_eq!(SinCos::index(-0.1), 504);
}

#[test]
fn particles_are_due_by_chopped_counts() {
    // 7.5 a second, on from 0 to 8: in the first second 7 are due, each
    // aged by how long ago it was due (never below 0).
    let ages = emission_ages(1.0, 0.0, 0.0, 8.0, 7.5);
    assert_eq!(ages.len(), 7);
    assert!((ages[0] - (1.0 - 1.0 / 7.5)).abs() < 1e-6);
    assert!(ages.iter().all(|&a| a >= 0.0));
    // The next frame: 1.0 → 1.1 chops to 7 and 8: one more.
    assert_eq!(emission_ages(1.1, 1.0, 0.0, 8.0, 7.5).len(), 1);
    // At most 15 at once.
    assert_eq!(emission_ages(8.0, 0.0, 0.0, 8.0, 7.5).len(), 15);
    // Nothing without a rate, before the stretch or after it.
    assert!(emission_ages(1.0, 0.0, 0.0, 8.0, 0.0).is_empty());
    assert!(emission_ages(9.0, 8.5, 0.0, 8.0, 7.5).is_empty());
    assert!(emission_ages(0.0, -1.0, 0.0, 8.0, 7.5).is_empty());
}

#[test]
fn controller_clocks_loop_through_their_keys() {
    let tc = TimeControl {
        flags: 0x48,
        frequency: 1.0,
        phase: 0.891,
        start: 0.0,
        stop: 8.0,
    };
    let range = Range { lo: 0.0, hi: 8.0 };
    let mut clock = Clock::default();
    // The first update counts from the time itself, plus the phase.
    let s = compute_scaled_time(&mut clock, &tc, range, 10.0);
    assert!((s - 2.891).abs() < 1e-5, "{s}");
    let s = compute_scaled_time(&mut clock, &tc, range, 15.5);
    assert!((s - 0.391).abs() < 1e-4, "{s}");
    // Clamped.
    let clamp = TimeControl { flags: 0x4C, ..tc };
    let mut clock = Clock::default();
    assert_eq!(compute_scaled_time(&mut clock, &clamp, range, 20.0), 8.0);
}

#[test]
fn step_keys_hold_until_the_next() {
    let interp = nif::particles::BoolInterp {
        value: 2,
        kind: 5,
        keys: vec![(0.0, true), (8.0, false)],
        timeline: false,
    };
    assert!(bool_at(&interp, -1.0));
    assert!(bool_at(&interp, 7.99));
    assert!(!bool_at(&interp, 8.0));
}

#[test]
fn colours_pack_as_the_game_sends_them() {
    assert_eq!(pack_color([1.0; 4]), [255; 4]);
    // The barracks dust's colour at full strength: bytes B, G, R, A.
    assert_eq!(
        pack_color([0.619_607_9, 0.650_980_4, 0.670_588_3, 0.45]),
        [171, 166, 158, 115]
    );
    // Not clamped: red past 1 (382.5, rounded half to even: 382 = 0x17E)
    // spills into alpha.
    assert_eq!(pack_color([1.5, 0.0, 0.0, 0.0]), [0, 0, 0x7E, 0x01]);
}

#[test]
fn sequences_wrap_hold_or_stay() {
    let s = ParticleSequence {
        name: "Idle".into(),
        start: 0.0,
        stop: 12.0,
        cycle: 0,
        frequency: 1.0,
        tracks: Vec::new(),
        unread_movers: Vec::new(),
    };
    assert_eq!(sequence_time(&s, 13.0, true), 1.0);
    assert_eq!(sequence_time(&s, 13.0, false), 0.0);
    let clamp = ParticleSequence {
        cycle: 2,
        ..s.clone()
    };
    assert_eq!(sequence_time(&clamp, 13.0, true), 12.0);
    let reverse = ParticleSequence { cycle: 1, ..s };
    assert_eq!(sequence_time(&reverse, 13.0, true), 11.0);
}

#[test]
fn the_barracks_dust_settles_at_59_as_recorded() {
    // The recording (call 26748565) drew 236 vertices: 59 particles, sides
    // 3.04–4.97, colour (158, 166, 171) at alpha up to 115.
    let placed = run(&Dust::default(), 20.0, false);
    let ps = placed.systems[0].active();
    assert!((58..=60).contains(&ps.len()), "{}", ps.len());
    for p in ps {
        let side = 2.0 * p.radius * p.size;
        assert!((3.0..=5.0).contains(&side), "{side}");
        let [b, g, r, a] = pack_color(p.color);
        assert_eq!((r, g, b), (158, 166, 171));
        assert!(a <= 115, "{a}");
        // Inside the cylinder (256 across and high) give or take their drift.
        assert!(p.position[0].hypot(p.position[1]) < 256.0 + 8.0 * 20.0);
    }
}

#[test]
fn nothing_is_emitted_on_the_first_frame() {
    let placed = run(&Dust::default(), 0.0, false);
    assert!(placed.systems[0].active().is_empty());
}

#[test]
fn a_manager_driven_emitter_needs_its_sequence() {
    let d = Dust {
        sequence: true,
        ..Dust::default()
    };
    assert!(run(&d, 3.0, false).systems[0].active().is_empty());
    let placed = run(&d, 3.0, true);
    let n = placed.systems[0].active().len();
    // 7.5 a second for about 3 s (the first frame only notes the time).
    assert!((20..=23).contains(&n), "{n}");
}

#[test]
fn gravity_pulls_by_strength_times_1_6() {
    let d = Dust {
        gravity: Some(100.0),
        speed: (0.0, 0.0),
        ..Dust::default()
    };
    let placed = run(&d, 2.0, false);
    let ps = placed.systems[0].active();
    assert!(ps.iter().any(|p| p.velocity[2] < -100.0));
    for p in ps {
        // Pulled down 160 units/s² since it was due (its age counts the
        // time before it was emitted twice: the game's first age step).
        assert!(p.velocity[2] <= 0.0);
        assert!(
            p.velocity[2] >= -160.0 * p.age - 1.0,
            "{} {}",
            p.velocity[2],
            p.age
        );
        assert!(p.velocity[2] <= -160.0 * p.age / 2.0 + 1.0);
    }
}

#[test]
fn emitters_follow_the_nodes_a_sequence_moves() {
    // A sequence holding the emitter's node 5000 units east: every
    // particle starts around there (the cylinder is 256 across).
    let (systems, sequences) = model(&Dust::default());
    let mut placed = PlacedParticles::new(&systems, sequences, Vec::new(), 3);
    let moved = nif::Sequence {
        name: "Idle".into(),
        start: 0.0,
        stop: 1.0,
        looping: true,
        tracks: vec![nif::Track {
            node: "Emitter".into(),
            motion: nif::Motion::Keys {
                translation: vec![(0.0, [5000.0, 0.0, 0.0]), (1.0, [5000.0, 0.0, 0.0])],
                rotation: Vec::new(),
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
    placed.set_motion(vec![(Arc::new(moved), true)]);
    for frame in 0..=120 {
        placed.update(frame as f32 / 60.0, &Transform::IDENTITY, [0.0; 3]);
    }
    let ps = placed.systems[0].active();
    assert!(!ps.is_empty());
    assert!(
        ps.iter().all(|p| p.position[0] > 4600.0),
        "{:?}",
        ps[0].position
    );
}

#[test]
fn quads_face_the_camera() {
    let placed = run(&Dust::default(), 2.0, false);
    let system = &placed.systems[0];
    let camera = CameraBasis {
        right: [1.0, 0.0, 0.0],
        up: [0.0, 0.0, 1.0],
        direction: [0.0, 1.0, 0.0],
    };
    let corners = system.quads(&camera, true);
    assert_eq!(corners.len(), 4 * system.active().len());
    // Back to front: the first quad is the farthest along the view.
    let depth = |k: usize| {
        let c = &corners[4 * k..4 * k + 4];
        c.iter().map(|v| v.position[1]).sum::<f32>() / 4.0
    };
    assert!(depth(0) >= depth(1));
    for q in corners.chunks(4) {
        // Flat in the picture's plane (y the view axis), centred on the
        // particle, the diagonals square to each other and s√2 long.
        assert!(q
            .iter()
            .all(|v| (v.position[1] - q[0].position[1]).abs() < 1e-3));
        let d = [0, 1, 2].map(|k| q[2].position[k] - q[0].position[k]);
        let e = [0, 1, 2].map(|k| q[3].position[k] - q[1].position[k]);
        let dot: f32 = (0..3).map(|k| d[k] * e[k]).sum();
        assert!(dot.abs() < 1e-2, "{dot}");
        assert_eq!(q[0].uv, [0.0, 1.0]);
        assert_eq!(q[2].uv, [1.0, 0.0]);
    }
}

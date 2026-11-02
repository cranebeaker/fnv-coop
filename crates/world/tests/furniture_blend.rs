//! Sitting down and getting up as the viewer drives them: the sit
//! procedure (`world::furniture::Sitter`) and the animations
//! (`world::animation::Player`) together, on generated sequences shaped
//! like Doc Mitchell's chair (`Chair_ForwardEnter.kf`: `Bip01` travels
//! 55.4 forward, `Bip01 NonAccum` turns half a turn, `Blend:15`;
//! `DynamicIdle_ChairSit.kf` faces half a turn from the entry's end;
//! `Chair_ForwardExit.kf` starts where the entry ended and, walking out
//! backwards, keeps that facing to within 10°, here exactly).
//!
//! The body's facing in the world is the actor's heading plus the
//! `NonAccum` turn. The game changes the heading by the marker's delta (or
//! half a turn) in the same frame as it swaps the animations without a
//! blend (`cSkipNextBlend`, `004974a0`), so the body never turns at once.
//! Before, the entry eased out over its 0.5 s after the heading had
//! turned: the body swung round half a turn and back ("Doc jumps in the
//! chair after sitting down").

use std::f32::consts::{PI, TAU};
use std::sync::Arc;

use esm::FormId;
use nif::anim::{Bone, Motion, Quat, Sequence, Track};
use nif::Transform;
use world::animation::{group, section, Player};
use world::furniture::{place_markers, Animations, MarkerSettings, SitState, Sitter, Step};

fn bones() -> Vec<Bone> {
    vec![
        Bone {
            name: "Bip01".into(),
            parent: None,
            local: Transform::IDENTITY,
        },
        Bone {
            name: "Bip01 NonAccum".into(),
            parent: Some(0),
            local: Transform::IDENTITY,
        },
    ]
}

/// A turn about the up axis.
fn yaw(a: f32) -> Quat {
    [(a / 2.0).cos(), 0.0, 0.0, (a / 2.0).sin()]
}

/// A sequence: `Bip01` travels `travel`, `NonAccum` turns through `turns`
/// (evenly over the length).
fn seq(
    name: &str,
    length: f32,
    looping: bool,
    travel: [f32; 3],
    turns: &[f32],
    keys: &[(f32, &str)],
) -> Arc<Sequence> {
    let n = (turns.len().max(2) - 1) as f32;
    Arc::new(Sequence {
        name: name.into(),
        start: 0.0,
        stop: length,
        looping,
        accum_root: Some("Bip01".into()),
        materials: Vec::new(),
        text_keys: keys.iter().map(|(t, k)| (*t, k.to_string())).collect(),
        tracks: vec![
            Track {
                node: "Bip01".into(),
                priority: 80,
                motion: Motion::Keys {
                    translation: vec![(0.0, [0.0; 3]), (length, travel)],
                    rotation: Vec::new(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            },
            Track {
                node: "Bip01 NonAccum".into(),
                priority: if looping { 35 } else { 80 },
                motion: Motion::Keys {
                    translation: Vec::new(),
                    rotation: turns
                        .iter()
                        .enumerate()
                        .map(|(i, a)| (length * i as f32 / n, yaw(*a)))
                        .collect(),
                    scale: Vec::new(),
                    default: (None, None, None),
                    euler: None,
                },
            },
        ],
    })
}

struct Chair {
    seat: Arc<Sequence>,
    enter: Arc<Sequence>,
    exit: Arc<Sequence>,
    stand: Arc<Sequence>,
}

impl Chair {
    fn new() -> Chair {
        Chair {
            seat: seq(
                "Seat",
                13.33,
                true,
                [0.0; 3],
                &[0.0, 0.0],
                &[(0.0, "start")],
            ),
            enter: seq(
                "Enter",
                1.733,
                false,
                [0.0, 55.4, 0.0],
                &[0.0, PI / 2.0, PI],
                &[(0.0, "start"), (0.5, "Blend:15"), (1.733, "end")],
            ),
            exit: seq(
                "Exit",
                1.533,
                false,
                [0.0, -55.4, 0.0],
                &[PI, PI],
                &[(0.0, "start"), (0.6, "Blend:15"), (1.533, "end")],
            ),
            stand: seq("Idle", 4.0, true, [0.0; 3], &[0.0, 0.0], &[(0.0, "start")]),
        }
    }

    fn get(&self, model: &str) -> &Arc<Sequence> {
        match model {
            "Seat" => &self.seat,
            "Enter" => &self.enter,
            "Exit" => &self.exit,
            _ => &self.stand,
        }
    }
}

impl Animations for &Chair {
    fn length(&mut self, model: &str) -> Option<f32> {
        let s = self.get(model);
        Some(s.stop - s.start)
    }
    fn root_offset(&mut self, model: &str, time: f32) -> [f32; 3] {
        self.get(model).root_offset(time).unwrap_or([0.0; 3])
    }
}

/// The body's facing: the heading plus the `NonAccum` turn.
fn facing(player: &Player, bones: &[Bone], heading: f32) -> f32 {
    let r = player.locals(bones)[1].rotation;
    (heading + r[1][0].atan2(r[0][0])).rem_euclid(TAU)
}

fn turned(a: f32, b: f32) -> f32 {
    let d = (a - b).rem_euclid(TAU);
    d.min(TAU - d)
}

/// Sits down and gets up as `sitting::furniture_frame` and
/// `ActorRig::drive` do, at 60 frames a second; the largest turn of the
/// body from one frame to the next.
fn largest_frame_turn(honour_skip: bool) -> f32 {
    let chair = Chair::new();
    let bones = bones();
    let front = nif::FurnitureMarker {
        offset: [-2.0, 62.8, -37.2],
        // Stored in thousandths of a radian.
        heading: 3141.0 / 1000.0,
        marker: 14,
    };
    let placed = place_markers(&[front], [0.0; 3], 0.0, 1.0)[0];
    let settings = MarkerSettings {
        delta: [2.4809, 57.3572, -28.948],
        heading_delta: PI,
    };
    let mut sitter = Sitter::new(FormId(1), placed, settings, placed.position, placed.heading);
    let mut pick = |sitting: u8, _: u8, _: u8| match sitting {
        1 => Some((FormId(2), "Seat".to_string())),
        2 => Some((FormId(3), "Enter".to_string())),
        4 => Some((FormId(4), "Exit".to_string())),
        _ => None,
    };
    let mut player = Player::default();
    player.play(group::IDLE, &chair.stand, -1, &bones);
    player.update(1.0);
    let dt = 1.0 / 60.0;
    let mut last = facing(&player, &bones, sitter.heading);
    let mut largest = 0.0f32;
    let mut seated_for = 0.0;
    for _ in 0..600 {
        if sitter.state == SitState::Sitting {
            seated_for += dt;
            if seated_for > 1.0 {
                sitter.stand_up();
            }
        }
        let mut anims = &chair;
        let step = sitter.update(dt, false, &mut pick, &mut anims);
        if std::mem::take(&mut sitter.skip_next_blend) && honour_skip {
            player.skip_next_blend();
        }
        // The rig: the seat's loop under the entry or exit, else standing.
        let busy = matches!(step, Step::Busy | Step::Settled);
        match (&sitter.dynamic_idle, busy) {
            (Some((_, m)), true) => player.play(group::DYNAMIC_IDLE, chair.get(m), -1, &bones),
            _ => player.play(group::IDLE, &chair.stand, -1, &bones),
        };
        match sitter.playing.as_ref().filter(|_| busy) {
            Some(p) => {
                player.play(group::SPECIAL_IDLE, chair.get(&p.model), -1, &bones);
            }
            None => player.stop_section(section::SPECIAL_IDLE),
        }
        player.update(dt);
        if let Some(p) = sitter.playing.as_ref().filter(|_| busy) {
            player.sync_time(section::SPECIAL_IDLE, p.elapsed);
        }
        let now = facing(&player, &bones, sitter.heading);
        largest = largest.max(turned(now, last));
        last = now;
        if step == Step::Released {
            break;
        }
    }
    assert_eq!(sitter.state, SitState::Normal, "got up again");
    largest
}

#[test]
fn the_body_never_turns_at_once_sitting_down_or_getting_up() {
    // The entry turns half a turn over 1.73 s, about 1.7° a frame, and
    // twice that at most while it blends in from the standing pose.
    let with = largest_frame_turn(true);
    assert!(with < 5.0f32.to_radians(), "{}°", with.to_degrees());
    // Blending across the heading change swings the body round.
    let without = largest_frame_turn(false);
    assert!(without > 90.0f32.to_radians(), "{}°", without.to_degrees());
}

/// Getting up waits for a seated idle only while it's starting (`00498f80`
/// in `00921e80` cases 4 and 9): not yet playing, or blending in. Once in,
/// however much of it is left, the exit begins. Doc Mitchell's
/// `SitChairRelaxA.kf` (16.1 s) used to hold his help-up in the opening
/// until it had played out, 16 s after his package changed (B14).
#[test]
fn a_seated_idle_holds_getting_up_only_while_it_blends_in() {
    let chair = Chair::new();
    let bones = bones();
    let front = nif::FurnitureMarker {
        offset: [-2.0, 62.8, -37.2],
        heading: 3141.0 / 1000.0,
        marker: 14,
    };
    let placed = place_markers(&[front], [0.0; 3], 0.0, 1.0)[0];
    let settings = MarkerSettings {
        delta: [2.4809, 57.3572, -28.948],
        heading_delta: PI,
    };
    let mut pick = |sitting: u8, _: u8, _: u8| match sitting {
        1 => Some((FormId(2), "Seat".to_string())),
        2 => Some((FormId(3), "Enter".to_string())),
        4 => Some((FormId(4), "Exit".to_string())),
        _ => None,
    };
    let mut sitter = Sitter::seated(FormId(1), placed, settings, 1.0, &mut pick);
    assert_eq!(sitter.state, SitState::Sitting);
    let mut player = Player::default();
    player.play(group::DYNAMIC_IDLE, &chair.seat, -1, &bones);
    player.update(1.0);
    // A seated idle of SitChairRelaxA.kf's length, not played yet: it
    // counts as starting (the game's idle whose sequence hasn't loaded).
    let relax = seq(
        "Relax",
        16.1,
        false,
        [0.0; 3],
        &[PI, PI],
        &[(0.0, "start"), (16.1, "end")],
    );
    assert!(player.idle_starting(section::SPECIAL_IDLE, &relax));
    player.play_idle_in(section::SPECIAL_IDLE, &relax, 0, &bones);
    sitter.stand_up();
    let dt = 1.0 / 60.0;
    let mut anims = &chair;
    // Blending in from the pose: getting up waits.
    let starting = player.idle_starting(section::SPECIAL_IDLE, &relax);
    assert!(starting);
    sitter.update(dt, starting, &mut pick, &mut anims);
    assert_eq!(sitter.state, SitState::Sitting);
    assert!(sitter.playing.is_none());
    // In at full weight (the default 0.2 s blend), nearly all of it left.
    for _ in 0..30 {
        player.update(dt);
    }
    let starting = player.idle_starting(section::SPECIAL_IDLE, &relax);
    assert!(!starting);
    assert!(player.time(section::SPECIAL_IDLE).unwrap() < 1.0);
    sitter.update(dt, starting, &mut pick, &mut anims);
    assert_eq!(sitter.state, SitState::WantToStand);
    assert_eq!(sitter.playing.as_ref().unwrap().model, "Exit");
}

//! Clutter Havok moves: the placed objects whose models carry a moving body
//! (`preview::cell::DynamicBody`), simulated by `physics::rigid` and drawn
//! where it has them.
//!
//! - Loaded places hand their bodies over as they're spawned ([`arrive`]);
//!   one that Havok moved before comes back where it came to rest
//!   (`GameState::havok_moved`, the game's "Havok moved" reference change).
//! - Each body's triangles are kept in the collider under its reference's
//!   form ID (walkers run into it, shots strike it) and moved with it.
//! - Shots striking a body push it as `Projectile::ApplyImpactForce` (Xbox
//!   PDB) does (`physics::impulses::projectile_impulse`, from
//!   `hiteffects::HitReports::shot_on_world`); explosions push the bodies in
//!   their sphere (`physics::impulses::explosion_push`, from
//!   `explosives`); the player, and people on their character controllers
//!   (`ai::move_body`, [`walkers`]), walking into one push it.
//! - The Grab control (Z) picks up the body under the crosshair and carries
//!   it on Havok's mouse spring (`physics::grab`).
//! - Contacts beginning play the game's impact sounds by Havok material
//!   (`physics::contacts`, `ImpactMixer::PlayCollisionSound` (Xbox PDB))
//!   and work out physics damage for destructible references (logged:
//!   destruction isn't implemented).
//! - Where a moved body is goes into the game state every frame it moves,
//!   so a save, or the place loading again, keeps it; one still moving
//!   keeps its velocities (`GameState::havok_velocity`).
//!
//! Not done: models with joined or several moving bodies (left solid where
//! placed: constraints aren't simulated), destructible objects' stages and
//! debris; impact sounds play without the game's attenuation and pitch
//! (`sounds` plays every sound flat).

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use bevy::prelude::*;
use cellview::space;
use esm::FormId;
use physics::contacts::{self, ContactSettings};
use physics::grab::{self, GrabSettings};
use physics::impulses::{self, ImpulseSettings};
use physics::proxy::SurfaceContact;
use physics::rigid::{ContactEvent, Pose, RigidWorld, Spring};
use preview::cell::DynamicBody;

use crate::controls::Controls;
use crate::dialogue::{Conversation, DialogueState};
use crate::menus::Menus;
use crate::scripts::PlacedRef;
use crate::walk::{game_point, CellCollision, Player};
use crate::{FlyCamera, GameFiles};

/// This viewer frame's time as the game's frame timer gives it to Havok
/// (`physics::havok::FrameTimer`, `00aa4ee0`): 0 on a viewer frame that
/// doesn't end a game frame. The bodies' world and the ragdolls step by
/// it, never by a viewer frame under the game's 10 ms.
#[derive(Resource, Default)]
pub(crate) struct HavokFrame {
    timer: physics::havok::FrameTimer,
    pub(crate) dt: f32,
}

fn time_havok_frame(time: Res<Time>, mut frame: ResMut<HavokFrame>) {
    let frame = &mut *frame;
    frame.dt = frame.timer.frame(time.delta_secs());
}

/// A push waiting for the simulation.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Push {
    /// A shot by `weapon` along `dir` striking the triangles of `owner` at
    /// `point`.
    Shot {
        weapon: FormId,
        shooter: FormId,
        dir: [f32; 3],
        point: [f32; 3],
        owner: u32,
    },
    /// An explosion of record `explosion` at `at`, reaching `radius`.
    Blast {
        explosion: world::explosions::ExplosionRecord,
        at: [f32; 3],
        radius: f32,
    },
}

/// Bodies of places just spawned, and pushes, for the next update (shared
/// as the throw queue in `explosives` is: no system parameters change).
static ARRIVED: Mutex<Vec<DynamicBody>> = Mutex::new(Vec::new());
static PUSHES: Mutex<Vec<Push>> = Mutex::new(Vec::new());
/// The people (not the player) walking this frame, as their character
/// controllers want to move (`ai::move_body`): they push what they walk
/// into as the player does.
static WALKERS: Mutex<Vec<SurfaceContact>> = Mutex::new(Vec::new());

/// The people's walkers for the next update (replacing the last list).
pub(crate) fn walkers(list: Vec<SurfaceContact>) {
    if let Ok(mut q) = WALKERS.lock() {
        *q = list;
    }
}

/// A spawned place's bodies, for the simulation.
pub(crate) fn arrive(bodies: &[DynamicBody]) {
    if bodies.is_empty() {
        return;
    }
    if let Ok(mut q) = ARRIVED.lock() {
        q.extend(bodies.iter().cloned());
    }
}

/// A shot striking the collider's triangle owned by `owner` (0: none).
pub(crate) fn shot(
    weapon: FormId,
    shooter: FormId,
    (eye, dir): ([f32; 3], [f32; 3]),
    distance: f32,
    owner: u32,
) {
    let point = [0, 1, 2].map(|k| eye[k] + dir[k] * distance);
    if owner == 0 {
        return;
    }
    if let Ok(mut q) = PUSHES.lock() {
        q.push(Push::Shot {
            weapon,
            shooter,
            dir,
            point,
            owner,
        });
    }
}

/// An explosion going off.
pub(crate) fn blast(explosion: &world::explosions::ExplosionRecord, at: [f32; 3], radius: f32) {
    if let Ok(mut q) = PUSHES.lock() {
        q.push(Push::Blast {
            explosion: explosion.clone(),
            at,
            radius,
        });
    }
}

/// The simulation, and what it needs to know about each body.
#[derive(Resource, Default)]
pub struct Clutter {
    pub world: RigidWorld,
    /// Each body's base's editor ID and its shapes' Havok materials.
    info: HashMap<u32, (String, Vec<u32>)>,
    /// Bodies whose drawing has been seen (one that's gone since was
    /// unloaded with its place).
    drawn: HashSet<u32>,
    settings: Option<ImpulseSettings>,
    contact_settings: Option<ContactSettings>,
    grab_settings: Option<GrabSettings>,
    /// Scripts' enable state when the bodies' triangles were last switched.
    disabled_seen: world::Disabled,
    /// The last of the player's proxy updates applied ([`Player::surface_serial`]).
    surface_seen: u32,
    /// What the player holds with the Grab control.
    held: Option<Held>,
    /// Bodies whose ground isn't in the collider yet.
    waiting: Vec<DynamicBody>,
    /// Bodies registered since draw last ran, for it to find their
    /// pieces.
    fresh: HashSet<u32>,
    /// When each pair of materials last sounded (seconds), for
    /// `iCollisionSoundTimeDelta`.
    sounded: HashMap<i32, f32>,
    /// Contact sounds to start ([`play_contact_sounds`]).
    contact_sounds: Vec<ContactSound>,
}

/// A contact's sound to start at its point (`00837550`).
#[derive(Debug, Clone, Copy)]
struct ContactSound {
    sound: FormId,
    at: [f32; 3],
    /// Hundredths of a decibel, in place of the sound's own.
    attenuation: i16,
    frequency: f32,
}

/// Whether a sound at `at` can be heard from `listener` (`00837550` →
/// `0082eca0`): on each axis no further than the sound's largest distance
/// byte × 100 (`00553b90`; the game doesn't put 0 up to 60 here).
// Translated from 0082eca0 (decompiled, FalloutNV.exe 1.4.0.525)
fn within_reach(max_byte: u8, at: [f32; 3], listener: [f32; 3]) -> bool {
    let reach = f32::from(max_byte) * 100.0;
    (0..3).all(|k| (at[k] - listener[k]).abs() <= reach)
}

/// Starts the contacts' sounds at their points, as loud as the listener's
/// distance makes them (`weapon_fx::play_at_with`, flags 0x4102: the
/// sound's own distances).
fn play_contact_sounds(
    mut commands: Commands,
    game: Res<GameFiles>,
    mut wavs: ResMut<Assets<crate::sounds::PcmSound>>,
    mut clutter: ResMut<Clutter>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut pick: Local<u64>,
) {
    if clutter.contact_sounds.is_empty() {
        return;
    }
    let sounds = std::mem::take(&mut clutter.contact_sounds);
    let Ok(camera) = cameras.single() else {
        return;
    };
    let listener = game_point(camera.translation);
    for s in sounds {
        *pick = pick.wrapping_add(1);
        crate::weapon_fx::play_at_with(
            &mut commands,
            &game.0,
            &mut wavs,
            s.sound,
            (s.at, listener),
            *pick,
            Some(s.attenuation),
            s.frequency,
        );
    }
}

/// The player's grab: the held reference and how far from the eye it's
/// held.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Held {
    reference: u32,
    distance: f32,
}

/// A setting from the INI (`[Audio]`…) or the game's settings.
fn setting(game: &cellview::Game, section: &str, name: &str) -> Option<f32> {
    game.settings
        .get(section, name)
        .and_then(|v| v.trim().parse().ok())
        .or_else(|| world::scripting::game_setting(&game.order, name))
}

/// `--physics engine`: the loose objects moved by the game's own Havok world
/// (`host::Host::run_ai_threads`: the frame's AI linear task threads, AI
/// thread 2 stepping the world in `TES::UpdateCellAnimations`), the viewer
/// only drawing where the game's nodes are. A frame the game's code can't
/// run, the viewer's own bodies step as before.
#[derive(Resource, Default)]
pub struct EngineWorld {
    mode: crate::args::PhysicsMode,
    /// This frame's step ran on the game's code.
    ran: bool,
    /// Each reference's root node pose when first seen, and now.
    start: HashMap<u32, Pose>,
    now: HashMap<u32, Pose>,
}

impl EngineWorld {
    pub fn new(mode: crate::args::PhysicsMode) -> Self {
        EngineWorld {
            mode,
            ..Default::default()
        }
    }

    /// The move of `reference`'s node since it was first seen: rotation
    /// `R = R1 R0ᵀ`, translation `t1 − R t0`.
    fn delta(&self, reference: u32) -> Option<Pose> {
        let (r0, t0) = self.start.get(&reference)?;
        let (r1, t1) = self.now.get(&reference)?;
        let mut r = [[0.0f32; 3]; 3];
        for (i, row) in r.iter_mut().enumerate() {
            for (j, v) in row.iter_mut().enumerate() {
                *v = (0..3).map(|k| r1[i][k] * r0[j][k]).sum();
            }
        }
        let t = [0, 1, 2].map(|i| t1[i] - (0..3).map(|k| r[i][k] * t0[k]).sum::<f32>());
        Some((r, t))
    }
}

/// The game's world step for this frame (`--physics engine`), before the
/// viewer's own (`simulate` skips its step on a frame this ran).
fn step_engine_world(
    havok: Res<HavokFrame>,
    mut engine: ResMut<EngineWorld>,
    host: Option<NonSendMut<host::Host>>,
) {
    engine.ran = false;
    if engine.mode != crate::args::PhysicsMode::Engine || havok.dt <= 0.0 {
        return;
    }
    let Some(mut host) = host else {
        return;
    };
    let Some(poses) = host.run_ai_threads(havok.dt) else {
        return;
    };
    engine.ran = true;
    for p in poses {
        let pose = (p.rotation, p.translation);
        engine.start.entry(p.reference.0).or_insert(pose);
        engine.now.insert(p.reference.0, pose);
    }
}

/// A drawn piece of a simulated object: its transform where the object
/// was put.
#[derive(Component)]
pub struct Simulated {
    reference: u32,
    rest: Mat4,
}

pub struct ClutterPlugin;

impl Plugin for ClutterPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Clutter>()
            .init_resource::<EngineWorld>()
            .init_resource::<HavokFrame>()
            // The frame timer: part of `Main::OnIdle_UpdateTimer` (step 32,
            // `crate::frame_order`).
            .add_systems(
                Update,
                time_havok_frame.in_set(crate::frame_order::FrameSet::step(
                    crate::frame_order::UPDATE_TIMER,
                )),
            )
            // The bodies' Havok step: `bhkWorld::Update` (slot +0xc4) on the
            // exterior world (`00554780`, through
            // `GridCellArray::UpdateManagedNodes`) or the interior's own
            // (`TESObjectCELL::UpdateManagedNodes`), between
            // `TES::LockHavokUpdateMT(1)` and `(0)` in
            // `TES::UpdateCellAnimations` (`00453550`), which an AI thread
            // calls after the actors' updates (`crate::frame_order::AiSet`):
            // under its gate, the bodies hold still in menu mode. After the
            // player's walk and attack (stage 2) by the stages' order.
            .add_systems(
                Update,
                (
                    // The Grab control's hold (`PlayerCharacter::Update`'s
                    // `0095f6c0`, the player stage's sub-step 43): kept here,
                    // before the step it feeds, until the player's systems
                    // read the crosshair inside the update.
                    grab_held
                        // Kept: thrown things land first.
                        .after(crate::explosives::fly_thrown)
                        .in_set(crate::frame_order::FrameSet::Stage(
                            world::frame::Stage::AiStart,
                        ))
                        .before(crate::frame_order::AiSet::Call(
                            crate::frame_order::UPDATE_CELL_ANIMATIONS,
                        )),
                    (step_engine_world, simulate, play_contact_sounds)
                        .chain()
                        .in_set(crate::frame_order::AiSet::Call(
                            crate::frame_order::UPDATE_CELL_ANIMATIONS,
                        ))
                        .after(crate::frame_order::AiSet::at(
                            crate::frame_order::UPDATE_CELL_ANIMATIONS,
                            crate::frame_order::HAVOK_LOCK_AT,
                        ))
                        .before(crate::frame_order::AiSet::at(
                            crate::frame_order::UPDATE_CELL_ANIMATIONS,
                            crate::frame_order::HAVOK_UNLOCK_AT,
                        )),
                    // The bodies drawn where they are, outside the gate (it
                    // picks up pieces as they are spawned, which it would miss
                    // under the gate).
                    draw.in_set(crate::frame_order::FrameSet::Stage(
                        world::frame::Stage::AiStart,
                    ))
                    .after(play_contact_sounds)
                    .before(crate::frame_order::AiSet::next(
                        crate::frame_order::UPDATE_CELL_ANIMATIONS,
                    )),
                ),
            );
    }
}

/// Takes arrivals and pushes, steps the bodies and keeps the collider and
/// the state up with them.
#[allow(clippy::too_many_arguments)]
fn simulate(
    time: Res<Time>,
    havok: Res<HavokFrame>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    mut collision: ResMut<CellCollision>,
    player: Res<Player>,
    mut clutter: ResMut<Clutter>,
    cameras: Query<&Transform, With<FlyCamera>>,
    drawn: Query<&Simulated>,
    (exterior, mut weathers, engine): (
        Option<Res<crate::exterior::Exterior>>,
        ResMut<crate::weather::Weathers>,
        Res<EngineWorld>,
    ),
) {
    // The game's own world stepped this frame (`--physics engine`): the
    // viewer's bodies wait (new arrivals stay queued for a frame it can't).
    if engine.ran {
        return;
    }
    let order = &game.0.order;
    // The sky's wind for the wind listener (`physics::wind`): outdoors the
    // weathers' mix, indoors none (`00453550`).
    let sky_wind = exterior.as_ref().map_or(0.0, |e| {
        weathers
            .mix(order, &state.0, e.weather)
            .map_or(0.0, |w| w.wind())
    });
    let state = &mut state.0;
    let clutter = &mut *clutter;
    let settings = *clutter.settings.get_or_insert_with(|| {
        ImpulseSettings::read(|name| world::scripting::game_setting(order, name))
    });
    let contact_settings = *clutter
        .contact_settings
        .get_or_insert_with(|| ContactSettings::read(|name| setting(&game.0, "Audio", name)));
    // New bodies (or ones whose place loaded again): where the state has
    // them, else where they're placed.
    let arrived: Vec<DynamicBody> = ARRIVED
        .lock()
        .map(|mut q| std::mem::take(&mut *q))
        .unwrap_or_default();
    // Ones waiting for their ground come first; a newer arrival of the
    // same reference replaces its wait.
    let mut queue: Vec<DynamicBody> = std::mem::take(&mut clutter.waiting);
    queue.retain(|w| !arrived.iter().any(|a| a.reference == w.reference));
    queue.extend(arrived);
    for body in queue {
        let reference = body.reference.0;
        // Not before the ground under it is in the collider: a square's
        // objects can arrive before its land and statics are merged in,
        // and a body settling then falls through the world (the game's
        // bodies live in the Havok world with their cell's ground; this
        // wait is the viewer's).
        let at = state
            .havok_moved
            .get(&body.reference)
            .map_or(body.pose.1, |p| p.1);
        let hidden = collision.0.is_hidden(reference);
        collision.0.set_hidden(reference, true);
        let ground = collision
            .0
            .raycast([at[0], at[1], at[2] + 64.0], [0.0, 0.0, -1.0], 4096.0);
        collision.0.set_hidden(reference, hidden);
        if ground.is_none() {
            clutter.waiting.push(body);
            continue;
        }
        if let Some(old) = clutter.world.find(reference) {
            clutter.world.remove_at(old);
        }
        if clutter.held.is_some_and(|h| h.reference == reference) {
            clutter.held = None;
        }
        clutter.drawn.remove(&reference);
        collision.0.set_hidden(reference, false);
        let i = clutter.world.add(body.setup, body.pose);
        if let Some(&pose) = state.havok_moved.get(&body.reference) {
            clutter.world.place(i, pose, true);
        }
        // Added asleep (`RigidWorld::add`): the game adds a loading
        // place's bodies to Havok in a batch without activating them
        // (`00c674d0` → `hkpWorld::addEntityBatch(…, 0)`, `00c94bd0`) and
        // wakes only those already moving; one added alone is left asleep
        // too when it's still and not on the biped layer (`00c6b0a0`). So
        // placed clutter stays where it was put until something touches
        // it. Still moving when saved: on its way again (`00563380`).
        if let Some(&(v, w)) = state.havok_velocity.get(&body.reference) {
            clutter.world.set_velocity(i, v, w);
        }
        if collision.0.owns(reference) {
            let (r, t) = clutter.world.delta(i);
            collision.0.move_owner(reference, &r, t);
        }
        clutter
            .info
            .insert(reference, (body.name.clone(), body.materials.clone()));
        clutter.fresh.insert(reference);
    }
    // Bodies whose drawing went (their place unloaded) leave.
    let shown: HashSet<u32> = drawn.iter().map(|s| s.reference).collect();
    let gone: Vec<u32> = clutter
        .drawn
        .iter()
        .filter(|r| !shown.contains(r))
        .copied()
        .collect();
    for r in gone {
        clutter.drawn.remove(&r);
        clutter.world.remove(r);
        clutter.info.remove(&r);
        if clutter.held.is_some_and(|h| h.reference == r) {
            clutter.held = None;
        }
    }
    clutter.drawn.extend(shown);
    if clutter.world.bodies.is_empty() {
        return;
    }
    // Each body's triangles in the collider (a new collider after the
    // loaded squares change has none), switched off while disabled (looked
    // at again when scripts enable or disable something).
    let recheck = clutter.disabled_seen != state.disabled;
    for i in 0..clutter.world.bodies.len() {
        let b = &clutter.world.bodies[i];
        let reference = b.setup.reference;
        let mut check = recheck;
        if !collision.0.owns(reference) {
            check = true;
            let (r0, t0) = b.rest();
            let materials = clutter
                .info
                .get(&reference)
                .map(|(_, m)| m.clone())
                .unwrap_or_default();
            let surface = physics::Surface {
                friction: b.setup.friction,
                restitution: b.setup.restitution,
            };
            for (k, shape) in b.setup.shapes.iter().enumerate() {
                let (v, t, shell) = shape.triangles();
                let placed: Vec<[f32; 3]> = v
                    .iter()
                    .map(|&p| {
                        let r = [0, 1, 2]
                            .map(|row| r0[row][0] * p[0] + r0[row][1] * p[1] + r0[row][2] * p[2]);
                        [r[0] + t0[0], r[1] + t0[1], r[2] + t0[2]]
                    })
                    .collect();
                let material = materials.get(k).copied().unwrap_or(physics::NO_MATERIAL);
                collision.0.add_layered(
                    &placed,
                    &t,
                    (shell, reference, material),
                    Some(surface),
                    b.setup.layer,
                );
            }
            let (r, t) = clutter.world.delta(i);
            collision.0.move_owner(reference, &r, t);
        }
        if check {
            let enabled = world::enabled_now(order, FormId(reference), &state.disabled);
            if collision.0.is_hidden(reference) == enabled {
                collision.0.set_hidden(reference, !enabled);
            }
        }
    }
    // (Kept only when it changed: it can hold thousands of references.)
    if recheck {
        clutter.disabled_seen = state.disabled.clone();
    }
    // The player's and the people's character proxies push the bodies they
    // touched in their last update (`applySurfaceInteractions`, `00cacf80`;
    // `ai::move_body` for the people), each update's touches once.
    let mut touches = Vec::new();
    if player.walking && player.ready && player.surface_serial != clutter.surface_seen {
        touches.extend(player.character.surface.iter().copied());
    }
    clutter.surface_seen = player.surface_serial;
    if let Ok(mut q) = WALKERS.lock() {
        touches.append(&mut q);
    }
    clutter
        .world
        .apply_surface_interactions(&touches, physics::havok::GRAVITY);
    if std::env::var("NV_PUSH_LOG").is_ok_and(|v| v == "1") && player.surface_serial % 30 == 1 {
        let f = player.character.feet;
        eprintln!(
            "player feet ({:.1}, {:.1}, {:.1}), walking {} ready {}, {} touches",
            f[0],
            f[1],
            f[2],
            player.walking,
            player.ready,
            player.character.surface.len()
        );
    }
    // `NV_PUSH_LOG=1`: each body the proxies touched, its speed after.
    if !touches.is_empty() && std::env::var("NV_PUSH_LOG").is_ok_and(|v| v == "1") {
        let mut seen = HashSet::new();
        for t in touches.iter().filter(|t| seen.insert(t.reference)) {
            if let Some(i) = clutter.world.find(t.reference) {
                let b = &clutter.world.bodies[i];
                eprintln!(
                    "proxy touches {:08X}: at ({:.1}, {:.1}, {:.1}), velocity ({:.1}, {:.1}, {:.1}), asleep {}",
                    t.reference,
                    b.center()[0],
                    b.center()[1],
                    b.center()[2],
                    b.velocity()[0],
                    b.velocity()[1],
                    b.velocity()[2],
                    b.asleep
                );
            }
        }
    }
    // Shots and blasts.
    let pushes: Vec<Push> = PUSHES
        .lock()
        .map(|mut q| std::mem::take(&mut *q))
        .unwrap_or_default();
    for push in pushes {
        apply(clutter, order, state, &settings, push);
    }
    // The step, and what moved.
    let awake: Vec<usize> = (0..clutter.world.bodies.len())
        .filter(|&i| !clutter.world.bodies[i].asleep)
        .collect();
    // Only on a game frame (`HavokFrame`): between them the game sleeps.
    if havok.dt > 0.0 {
        clutter.world.update(&collision.0, havok.dt);
    }
    // The wind listener, once a game frame after the steps, with its
    // time (`00c6ae70` → `00c66e20`), on the bodies the wind moves.
    let dt = havok.dt;
    let wind = physics::wind::Wind::set(sky_wind, physics::wind::SKY_WIND_DIRECTION);
    if dt > 0.0 && wind.speed != 0.0 {
        // Every five seconds, where the wind has taken its bodies.
        let now = time.elapsed_secs();
        if (now / 5.0).floor() != ((now - dt) / 5.0).floor() {
            for b in clutter.world.bodies.iter().filter(|b| b.setup.wind) {
                let at = b.pose().1;
                println!(
                    "{now:.1} s: wind {:.1} (heading {:.2} rad) has {} (quality {}) at ({:.1}, {:.1}, {:.1}), {:.1} units from where it was put.",
                    wind.speed,
                    wind.direction,
                    FormId(b.setup.reference),
                    b.setup.quality,
                    at[0],
                    at[1],
                    at[2],
                    dist(at, b.rest().1)
                );
            }
        }
        for i in 0..clutter.world.bodies.len() {
            if !clutter.world.bodies[i].setup.wind {
                continue;
            }
            // `GetRandom(x)`: −x..x from the game's dice (`00476b70`).
            let mut random = |x: f32| x * ((state.roll() % 2_000_001) as f32 / 1_000_000.0 - 1.0);
            if let Some(force) = physics::wind::push(&wind, dt, &mut random) {
                clutter.world.apply_force(i, force, dt);
            }
        }
    }
    let woken = (0..clutter.world.bodies.len()).filter(|&i| !clutter.world.bodies[i].asleep);
    let moving: std::collections::BTreeSet<usize> = awake.into_iter().chain(woken).collect();
    for i in moving {
        let b = &clutter.world.bodies[i];
        let reference = b.setup.reference;
        let (r, t) = clutter.world.delta(i);
        collision.0.move_owner(reference, &r, t);
        if b.moved {
            state.havok_moved.insert(FormId(reference), b.pose());
        }
        // An active body's velocities go with it into a save (`00563220`).
        if b.asleep {
            state.havok_velocity.remove(&FormId(reference));
        } else if b.moved {
            state
                .havok_velocity
                .insert(FormId(reference), (b.velocity(), b.spin()));
        }
        if b.asleep {
            let name = clutter.info.get(&reference).map_or("", |(n, _)| n.as_str());
            let at = b.pose().1;
            let put = b.rest().1;
            println!(
                "{:.1} s: {} ({name}) comes to rest at ({:.1}, {:.1}, {:.1}), {:.1} units from where it was put.",
                time.elapsed_secs(),
                FormId(reference),
                at[0],
                at[1],
                at[2],
                dist(at, put)
            );
        }
    }
    // Contacts begun: impact sounds and physics damage.
    let now = time.elapsed_secs();
    let listener = cameras.single().map_or(player.character.feet, |camera| {
        game_point(camera.translation)
    });
    for event in clutter.world.take_contacts() {
        contact(
            clutter,
            &collision.0,
            order,
            &contact_settings,
            now,
            listener,
            event,
        );
    }
}

/// One side of a contact: its Havok material (`-1`: none), its mass for
/// the sounds (`contacts::STATIC_SIDE_MASS` without a moving body) and for
/// the damage (`contacts::FIXED_MASS` for a fixed one), and its reference.
fn side(clutter: &Clutter, body: usize) -> (i32, f32, u32) {
    let b = &clutter.world.bodies[body];
    let reference = b.setup.reference;
    let material = clutter
        .info
        .get(&reference)
        .and_then(|(_, m)| m.first().copied())
        .filter(|&m| m != physics::NO_MATERIAL)
        .map_or(-1, |m| m as i32);
    (material, b.setup.mass, reference)
}

/// A contact beginning (`FOCollisionListener::contactPointAddedCallback`
/// (Xbox PDB), `00623cb0`): the impact sounds of both sides by material
/// (`physics::contacts::collision_sound`), one per pair of materials per
/// `iCollisionSoundTimeDelta`; then physics damage for a destructible
/// reference struck hard enough (`006238b0`: a reference with
/// destructible data, not yet destroyed).
#[allow(clippy::too_many_arguments)]
fn contact(
    clutter: &mut Clutter,
    collider: &physics::Collider,
    order: &esm::LoadOrder,
    s: &ContactSettings,
    now: f32,
    listener: [f32; 3],
    e: ContactEvent,
) {
    let (mat_a, mass_a, ref_a) = side(clutter, e.body);
    let (mat_b, sound_mass_b, damage_mass_b, ref_b) = match (e.other_body, e.triangle) {
        (Some(j), _) => {
            let (m, mass, r) = side(clutter, j);
            (m, mass, mass, r)
        }
        (None, Some(t)) => (
            collider.material(t).map_or(-1, |m| m as i32),
            contacts::STATIC_SIDE_MASS,
            contacts::FIXED_MASS,
            collider.owner(t),
        ),
        _ => return,
    };
    let name = |r: u32| {
        clutter
            .info
            .get(&r)
            .map_or_else(|| FormId(r).to_string(), |(n, _)| n.clone())
    };
    if contacts::audible(e.speed, s) {
        if let Some(sound) =
            contacts::collision_sound((mat_a, mass_a), (mat_b, sound_mass_b), e.speed, s)
        {
            // Each side's sound record, then (`00837550`) nothing unless
            // one is within reach of the listener, then one per pair of
            // materials per `iCollisionSoundTimeDelta`.
            let forms: Vec<(&str, FormId, u8)> = sound
                .sounds
                .into_iter()
                .flatten()
                .filter_map(|id| {
                    let form = order.form_by_editor_id(id)?;
                    let max = world::weapon_fx::SoundLevels::load(order, form)?.max_byte;
                    Some((id, form, max))
                })
                .collect();
            let heard = forms
                .iter()
                .any(|&(_, _, max)| within_reach(max, e.point, listener));
            let gap = s.time_delta_ms as f32 / 1000.0;
            let quiet = clutter
                .sounded
                .get(&sound.key)
                .is_some_and(|&t| now - t < gap);
            if heard && !quiet {
                clutter.sounded.insert(sound.key, now);
                let mut played = Vec::new();
                for (id, form, _) in forms {
                    clutter.contact_sounds.push(ContactSound {
                        sound: form,
                        at: e.point,
                        attenuation: sound.attenuation as i16,
                        frequency: sound.frequency,
                    });
                    played.push(id);
                }
                println!(
                    "  {} meets {} at {:.0} units a second: {} (attenuation {:.1} dB, frequency × {:.2})",
                    name(ref_a),
                    name(ref_b),
                    e.speed,
                    played.join(" + "),
                    f32::from(sound.attenuation) / 100.0,
                    sound.frequency
                );
            }
        }
    }
    // Physics damage: Havok's speed, each side by the other's mass.
    let havok_speed = e.speed / physics::HAVOK_UNIT;
    if !contacts::damaging(havok_speed, s) {
        return;
    }
    for (victim, other_mass) in [(ref_a, damage_mass_b), (ref_b, mass_a)] {
        if victim == 0 || !destructible(order, FormId(victim)) {
            continue;
        }
        let damage = contacts::physics_damage(other_mass, havok_speed, s);
        if damage > 0.0 {
            println!(
                "  physics damage {damage:.1} to {} (destruction isn't implemented)",
                name(victim)
            );
        }
    }
}

/// Whether a reference's base carries destructible data (`DEST`): the
/// references the game's physics damage goes to (the form flag
/// 0x1000000 that `006238b0` tests, read with the destructible data in
/// `00579220`; inferred to mean "has destructible data").
fn destructible(order: &esm::LoadOrder, reference: FormId) -> bool {
    world::scripting::base_of(order, reference)
        .and_then(|b| order.get(b))
        .and_then(|rr| rr.record().ok())
        .is_some_and(|r| r.get(esm::FourCC::new(b"DEST")).is_some())
}

/// The Grab control: takes hold of the body under the crosshair, carries
/// it, lets it go (`physics::grab`; `PlayerCharacter::HandlePhysicsGrab`,
/// `::UpdateMouseSpring` (Xbox PDB), `0095f6c0`, `00960520`).
#[allow(clippy::too_many_arguments)]
fn grab_held(
    game: Res<GameFiles>,
    // The Grab control is the player's (`PlayerCharacter::Update`, the grab
    // controls at `0094363c`): the controls' buffer as the player's step
    // read it (`crate::poll`), though this runs after the poll for order.
    player_input: Res<crate::poll::PlayerInput>,
    controls: Res<Controls>,
    player: Res<Player>,
    (conversation, menus): (Res<Conversation>, Res<Menus>),
    mut collision: ResMut<CellCollision>,
    mut clutter: ResMut<Clutter>,
    cameras: Query<&Transform, With<FlyCamera>>,
    crosshair: Res<crate::crosshair::Crosshair>,
) {
    let clutter = &mut *clutter;
    let busy = !player.walking || !player.ready || conversation.0.is_some() || menus.is_open();
    let Ok(camera) = cameras.single() else {
        return;
    };
    let eye = game_point(camera.translation);
    let f = camera.forward().as_vec3();
    let view = [f.x, -f.z, f.y];
    let s = *clutter.grab_settings.get_or_insert_with(|| {
        GrabSettings::read(|name| world::scripting::game_setting(&game.0.order, name))
    });
    let pressed = !busy
        && controls
            .grab
            .just_pressed(&player_input.keys, &player_input.buttons);
    if let Some(held) = clutter.held {
        let Some(i) = clutter.world.find(held.reference) else {
            clutter.held = None;
            clutter.world.spring = None;
            return;
        };
        if pressed || collision.0.is_hidden(held.reference) {
            println!("Let go of {}.", FormId(held.reference));
            clutter.held = None;
            clutter.world.spring = None;
            return;
        }
        if busy {
            return;
        }
        // The target: along the view, short of anything else in the way.
        collision.0.set_hidden(held.reference, true);
        let blocked = collision
            .0
            .raycast(eye, view, held.distance)
            .map(|(d, _)| d);
        collision.0.set_hidden(held.reference, false);
        let target = grab::target(eye, view, held.distance, blocked);
        let Some(spring) = clutter.world.spring.as_mut() else {
            clutter.held = None;
            return;
        };
        spring.target = target;
        let local = spring.local;
        let point = clutter.world.point(i, local);
        let gap = dist(point, target);
        let heaviest = clutter.world.heaviest_contact(i);
        let mass = clutter.world.bodies[i].setup.mass;
        if grab::lets_go(gap, heaviest, mass, &s) {
            println!(
                "Lost hold of {} ({gap:.0} units from where it's held).",
                FormId(held.reference)
            );
            clutter.held = None;
            clutter.world.spring = None;
        }
        return;
    }
    if !pressed {
        return;
    }
    // What the crosshair is on (`crosshair`, the game's view caster), and
    // its body.
    let Some(hit) = crosshair.0 else {
        return;
    };
    let owner = hit.reference;
    let Some(i) = clutter.world.find(owner) else {
        return;
    };
    let b = &clutter.world.bodies[i];
    if !grab::may_grab(b.setup.mass, impulses::moves(b.setup.motion), &s) {
        println!(
            "{} is too heavy to grab ({} > {}).",
            FormId(owner),
            b.setup.mass,
            s.max_weight
        );
        return;
    }
    let (damping, elasticity, object_damping, max_force) = grab::spring_values(b.setup.layer, &s);
    let distance =
        grab::hold_distance(dist(hit.point, eye), physics::CharacterShape::PLAYER.radius);
    let local = clutter.world.local_point(i, hit.point);
    clutter.world.spring = Some(Spring {
        body: i,
        local,
        target: hit.point,
        damping,
        elasticity,
        max_relative_force: max_force,
        object_damping,
    });
    clutter.world.wake(i);
    clutter.held = Some(Held {
        reference: owner,
        distance,
    });
    println!(
        "Grabbed {} ({}) {distance:.0} units away.",
        FormId(owner),
        clutter.info.get(&owner).map_or("", |(n, _)| n.as_str())
    );
}

/// The projectile a weapon's shot is: its ammunition's (`AMMO` `DAT2` form
/// at 4) when it names one, else the weapon's own (as `world::vats` reads
/// it).
fn fired_projectile(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    shooter: FormId,
    weapon: &world::combat::Weapon,
) -> Option<FormId> {
    let from_ammo = weapon
        .ammo_in_use(order, state, shooter)
        .and_then(|a| order.get(a))
        .and_then(|rr| {
            let d = rr
                .record()
                .ok()?
                .get(esm::FourCC::new(b"DAT2"))?
                .data
                .clone();
            (d.len() >= 8)
                .then(|| {
                    rr.plugin
                        .to_global(FormId(u32::from_le_bytes([d[4], d[5], d[6], d[7]])))
                })
                .filter(|f| f.0 != 0)
        });
    from_ammo.or(weapon.projectile)
}

/// One push on the bodies.
fn apply(
    clutter: &mut Clutter,
    order: &esm::LoadOrder,
    state: &mut world::scripting::GameState,
    settings: &ImpulseSettings,
    push: Push,
) {
    match push {
        Push::Shot {
            weapon,
            shooter,
            dir,
            point,
            owner,
        } => {
            let Some(i) = clutter.world.find(owner) else {
                return;
            };
            let name = clutter.info.get(&owner).map_or("", |(n, _)| n.as_str());
            let Some(w) = world::combat::Weapon::load(order, weapon) else {
                println!(
                    "  the shot strikes {} ({name}): no weapon record",
                    FormId(owner)
                );
                return;
            };
            let Some(projectile) = fired_projectile(order, state, shooter, &w)
                .and_then(|p| world::explosions::ProjectileRecord::load(order, p))
            else {
                println!(
                    "  the shot strikes {} ({name}): no projectile",
                    FormId(owner)
                );
                return;
            };
            // `Projectile::ProcessImpacts` (`009c1b70`) leaves the push to
            // the explosion for projectiles that go off on impact.
            if projectile.explodes() && !projectile.alt_trigger() {
                return;
            }
            let b = &clutter.world.bodies[i];
            if !impulses::moves(b.setup.motion) {
                return;
            }
            let Some(j) = impulses::projectile_impulse(
                projectile.impact_force,
                dir,
                b.setup.layer,
                b.setup.mass,
                settings,
            ) else {
                println!(
                    "  the shot strikes {} ({name}): {} has no impact force",
                    FormId(owner),
                    projectile.form_id
                );
                return;
            };
            println!(
                "  the shot pushes {} ({}): impact force {} → impulse {:.1} (layer {}, mass {})",
                FormId(owner),
                clutter.info.get(&owner).map_or("", |(n, _)| n.as_str()),
                projectile.impact_force,
                (j[0] * j[0] + j[1] * j[1] + j[2] * j[2]).sqrt(),
                b.setup.layer,
                b.setup.mass
            );
            clutter.world.apply_point_impulse(i, j, point);
        }
        Push::Blast {
            explosion,
            at,
            radius,
        } => {
            let push_source_only =
                explosion.flags & world::explosions::expl_flags::PUSH_SOURCE_ONLY != 0;
            for i in 0..clutter.world.bodies.len() {
                let b = &clutter.world.bodies[i];
                // In the explosion's sphere (`Explosion::InitHavok`, a phantom
                // of its radius): taken here as the body's centre within the
                // radius and its reach.
                let d = {
                    let c = b.center();
                    ((c[0] - at[0]).powi(2) + (c[1] - at[1]).powi(2) + (c[2] - at[2]).powi(2))
                        .sqrt()
                };
                if d > radius {
                    continue;
                }
                // The explosion's source reference (`+0xcc`) isn't tracked
                // here: no body counts as the source's.
                if !impulses::explosion_pushes(
                    b.setup.motion,
                    b.setup.layer,
                    push_source_only,
                    false,
                ) {
                    continue;
                }
                let mut random = || {
                    // −1..1 from the state's dice (`00476b70`).
                    (state.roll() % 2_000_001) as f32 / 1_000_000.0 - 1.0
                };
                let Some((linear, angular)) = impulses::explosion_push(
                    explosion.force,
                    at,
                    b.center(),
                    b.setup.layer,
                    b.setup.mass,
                    false,
                    settings,
                    &mut random,
                ) else {
                    continue;
                };
                clutter.world.apply_linear_impulse(i, linear);
                clutter.world.apply_angular_impulse(i, angular);
            }
        }
    }
}

fn dist(a: [f32; 3], b: [f32; 3]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// A game-space rigid move as one in Bevy's space (meters, y up).
fn bevy_delta((r, t): Pose) -> Mat4 {
    let mut m = [0.0f32; 16];
    for col in 0..3 {
        for row in 0..3 {
            m[col * 4 + row] = r[row][col];
        }
    }
    m[12] = t[0];
    m[13] = t[1];
    m[14] = t[2];
    m[15] = 1.0;
    let identity = Mat4::IDENTITY.to_cols_array();
    let to_view = Mat4::from_cols_array(&space::matrix(&identity));
    to_view * Mat4::from_cols_array(&m) * to_view.inverse()
}

/// A drawn piece just spawned: its entity, reference and transform.
type NewPiece = (Entity, Ref<'static, PlacedRef>, &'static Transform);

/// Draws simulated objects where their bodies are.
fn draw(
    mut commands: Commands,
    mut clutter: ResMut<Clutter>,
    engine: Res<EngineWorld>,
    new: Query<NewPiece, Without<Simulated>>,
    mut pieces: Query<(&Simulated, &mut Transform)>,
) {
    // Pieces of bodies registered this frame (some after waiting for
    // their ground, their drawing spawned frames before).
    let fresh = std::mem::take(&mut clutter.fresh);
    for (entity, placed, transform) in new.iter().filter(|(_, p, _)| {
        p.is_added() || fresh.contains(&p.0) || (engine.ran && engine.now.contains_key(&p.0))
    }) {
        if clutter.world.find(placed.0).is_some() || engine.now.contains_key(&placed.0) {
            commands.entity(entity).insert(Simulated {
                reference: placed.0,
                rest: transform.compute_matrix(),
            });
        }
    }
    for (piece, mut transform) in &mut pieces {
        // Where the game's nodes are on a frame its world stepped, else
        // where the viewer's bodies are.
        let delta = if engine.ran {
            engine.delta(piece.reference)
        } else {
            clutter
                .world
                .find(piece.reference)
                .map(|i| clutter.world.delta(i))
        };
        let Some(delta) = delta else {
            continue;
        };
        let wanted = Transform::from_matrix(bevy_delta(delta) * piece.rest);
        if *transform != wanted {
            *transform = wanted;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deltas_move_drawings_as_they_move_bodies() {
        // A quarter turn about the game's z and a move east and up: a point
        // drawn at the game's (10, 0, 0) goes to (0, 10, 0) + (5, 0, 2).
        let r = [[0.0, -1.0, 0.0], [1.0, 0.0, 0.0], [0.0, 0.0, 1.0]];
        let m = bevy_delta((r, [5.0, 0.0, 2.0]));
        let drawn = Vec3::from(space::point([10.0, 0.0, 0.0]));
        let moved = m.transform_point3(drawn);
        let want = Vec3::from(space::point([5.0, 10.0, 2.0]));
        assert!((moved - want).length() < 1e-5, "{moved} vs {want}");
    }

    #[test]
    fn contact_sounds_are_heard_only_within_the_sounds_reach() {
        // A tumbleweed's rattle (largest distance byte 20: 2000 units) far
        // off in the wind is silent; one beside the listener isn't. The
        // reach is a box, axis by axis, and a byte of 0 reaches nowhere.
        let listener = [-68233.0, 4990.0, 8400.0];
        assert!(within_reach(20, [-68233.0, 6900.0, 8400.0], listener));
        assert!(!within_reach(20, [-64960.0, -4238.0, 8400.0], listener));
        assert!(within_reach(20, [-66300.0, 6900.0, 8400.0], listener));
        assert!(!within_reach(0, [-68233.0, 4991.0, 8400.0], listener));
    }

    #[test]
    fn shots_are_queued_only_for_owned_triangles() {
        let before = PUSHES.lock().map(|q| q.len()).unwrap_or(0);
        shot(
            FormId(1),
            FormId(0x14),
            ([0.0; 3], [1.0, 0.0, 0.0]),
            50.0,
            0,
        );
        assert_eq!(PUSHES.lock().map(|q| q.len()).unwrap_or(0), before);
        shot(
            FormId(1),
            FormId(0x14),
            ([0.0; 3], [1.0, 0.0, 0.0]),
            50.0,
            0x77,
        );
        let q = PUSHES.lock().unwrap();
        assert!(q.iter().any(|p| matches!(
            p,
            Push::Shot { owner: 0x77, point, .. } if point[0] == 50.0
        )));
    }
}

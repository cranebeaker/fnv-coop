//! Weapon fire effects on screen and in the speakers, by the rules in
//! `world::weapon_fx` (read from the game's code: see that module).
//!
//! - **Firing sounds**: the player's and people's shots (`combat`,
//!   `fighting`) and objects' `FireWeapon` shots report here; each plays
//!   the sounds `world::weapon_fx::plan_fire_sounds` picks (the player's 2D
//!   sound, everyone else's 3D and distant ones), placed at the weapon's
//!   `ProjectileNode` (else the shooter) and following the shooter, as loud
//!   as the game's DirectSound volume works out every frame from the
//!   listener's distance (`world::weapon_fx::amplitude`), within the 20
//!   entries the game keeps. The listener is the camera. Not placed left or
//!   right (DirectSound's 3D panning isn't done here).
//! - **Melee swings** that meet no one play the weapon's `TNAM` sound at the
//!   attacker (`00899200`), as loud as the sound's own distances say.
//! - **Muzzle flashes**: a projectile with a muzzle flash lights the
//!   shooter's (`world::weapon_fx::MuzzleFlash`): its `NAM1` model drawn at
//!   the weapon's `ProjectileNode` for the flash's duration, in the
//!   first-person view for the player in first person, on the third-person
//!   body or on people otherwise, kept facing the camera where the model's
//!   billboards do.
//!
//! Not done: the flash's light (`PROJ` `DATA` 20; the viewer's lights are
//! fixed on each surface when a place loads), the flash model's own
//! animations and particles (it is drawn as it stands; particles are left
//! out), flashes on objects that fire (`FireWeapon`), the Turbo and
//! V.A.T.S. time scales on the flash's clock, projectiles in flight and
//! tracers (the viewer's shots are rays; `world::weapon_fx::
//! ProjectileEffects::tracer` is the game's roll), and the anim actions
//! that keep a flash dark (9, 15–17). Impacts are `hiteffects` (B6).

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bevy::audio::{AudioSink, AudioSinkPlayback, PlaybackSettings, Volume};
use bevy::prelude::*;
use bevy::render::view::{NoFrustumCulling, RenderLayers};
use cellview::space;
use esm::FormId;
use world::dialogue::PLAYER_REF;
use world::weapon_fx::{
    self, FireSoundMods, FireSoundSlots, MuzzleFlash, ProjectileEffects, SoundLevels, WeaponSounds,
};

use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::dialogue::DialogueState;
use crate::sounds::PcmSound;
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles, Spawner};

/// A shot fired: who, with what, and from where when the shooter isn't
/// someone on screen (an object's `FireWeapon`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fired {
    pub shooter: FormId,
    pub weapon: FormId,
    pub from: Option<[f32; 3]>,
}

static SHOTS: Mutex<Vec<Fired>> = Mutex::new(Vec::new());
static SWINGS: Mutex<Vec<(FormId, FormId)>> = Mutex::new(Vec::new());

/// Reports a shot (a gun's attack, `00523150`) for its sounds and flash.
pub(crate) fn fired(shot: Fired) {
    if let Ok(mut q) = SHOTS.lock() {
        q.push(shot);
    }
}

/// Reports a melee swing by `attacker` with `weapon` that met no one
/// (`00899200`).
pub(crate) fn swung(attacker: FormId, weapon: FormId) {
    if let Ok(mut q) = SWINGS.lock() {
        q.push((attacker, weapon));
    }
}

/// A firing or swing sound playing: what it follows and how loud.
#[derive(Component)]
pub struct PlacedSound {
    shooter: FormId,
    /// Where it sits relative to the shooter's feet when it started (the
    /// fire node's offset), or where it sits for good (no shooter on
    /// screen).
    offset: [f32; 3],
    follows: bool,
    distances: (f32, f32),
    curve: [u16; 5],
    static_attenuation: i16,
    volume: f32,
}

/// The muzzle flash pieces: the model's piece in its own space (game
/// units), its billboard, and the point it's built around.
struct FlashPiece {
    entity: Entity,
    local: Mat4,
    billboard: Option<cellview::Billboard>,
    center: Vec3,
}

/// One shooter's muzzle flash (the game's per-process flash, `+0x3d4`).
struct Flash {
    state: MuzzleFlash,
    projectile: FormId,
    model: String,
    first_person: bool,
    pieces: Vec<FlashPiece>,
    /// The weapon's `ProjectileNode` in its model.
    node: Option<nif::Transform>,
}

/// The effects' state: the firing sound entries and their sounds, the
/// flashes, and what's been read.
#[derive(Resource, Default)]
pub struct WeaponEffects {
    slots: FireSoundSlots,
    slot_entities: [Vec<Entity>; weapon_fx::FIRE_SOUND_SLOTS],
    flashes: HashMap<(FormId, bool), Flash>,
    scenes: HashMap<String, Option<Arc<cellview::ViewerScene>>>,
    nodes: HashMap<String, Option<nif::Transform>>,
    levels: HashMap<FormId, Option<SoundLevels>>,
    /// A flash on screen this frame that's still lit next frame (for
    /// `NV_SHOT_ON_FLASH`): `Some(true)` the player's, `Some(false)`
    /// someone else's in front of the camera.
    pub shown: Option<bool>,
}

impl WeaponEffects {
    fn levels(&mut self, order: &esm::LoadOrder, id: FormId) -> Option<SoundLevels> {
        *self
            .levels
            .entry(id)
            .or_insert_with(|| SoundLevels::load(order, id))
    }

    fn node(&mut self, game: &cellview::Game, model: &str) -> Option<nif::Transform> {
        *self
            .nodes
            .entry(model.to_ascii_lowercase())
            .or_insert_with(|| {
                weapon_fx::FIRE_NODE_NAMES
                    .iter()
                    .find_map(|n| crate::viewmodel::node_in_model(game, model, n))
            })
    }
}

pub struct WeaponEffectsPlugin;

impl Plugin for WeaponEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WeaponEffects>()
            .add_systems(
                Update,
                (start_effects, update_flashes, attenuate)
                    .chain()
                    // In the AI work (stage 6, `crate::frame_order`): after the
                    // player's stage (attack, objects' shots, the view model) by
                    // the stages' order. Kept: after people's moves. Placed for
                    // order only, after the actors' movement pass
                    // (`crate::frame_order::AiSet`), outside its gate (the
                    // flashes' and firing sounds' exact place isn't traced).
                    .after(crate::ai::move_actors)
                    .in_set(crate::frame_order::FrameSet::Stage(
                        world::frame::Stage::AiStart,
                    ))
                    .after(crate::frame_order::AiSet::Call(
                        crate::frame_order::ACTORS_MOVEMENT,
                    ))
                    .before(crate::frame_order::AiSet::next(
                        crate::frame_order::ACTORS_MOVEMENT,
                    )),
            )
            .add_systems(
                PostUpdate,
                place_flashes.after(bevy::transform::TransformSystem::TransformPropagate),
            );
    }
}

/// The `Weapon` joint of the shooter's skeleton on screen: the player's
/// first-person one (in first person) or third-person body, or someone's.
type Rigs<'w, 's> = Query<'w, 's, (Entity, &'static ActorRig, Option<&'static Walker>)>;

fn weapon_joint(
    shooter: FormId,
    first_person: bool,
    view: &crate::viewmodel::ViewModel,
    body: &crate::player_body::PlayerBody,
    rigs: &Rigs,
) -> Option<Entity> {
    let rig = if shooter == PLAYER_REF {
        if first_person {
            return view.weapon_joint();
        }
        rigs.get(body.root()?).ok()?.1
    } else {
        rigs.iter()
            .find(|(_, _, w)| w.is_some_and(|w| w.reference == shooter))?
            .1
    };
    let bone = rig
        .skeleton
        .bones
        .iter()
        .position(|b| b.name.eq_ignore_ascii_case("Weapon"))?;
    rig.joints.get(bone).copied()
}

/// Bevy's space from the game's (axes and units).
fn to_view() -> Mat4 {
    Mat4::from_cols_array(&space::matrix(&Mat4::IDENTITY.to_cols_array()))
}

/// Where the weapon's fire node is in the game's space, from its joint's
/// placement on screen and the node's place in the weapon's model.
fn fire_node_world(joint: &GlobalTransform, node: &nif::Transform) -> Mat4 {
    to_view().inverse()
        * joint.compute_matrix()
        * crate::actors::bevy_transform(node).compute_matrix()
}

/// Whether the player is seen in first person now.
fn in_first_person(view: &crate::player_camera::PlayerView) -> bool {
    !view.camera.actually_third
}

/// The weapon's model as the shooter holds it.
fn held_model(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    shooter: FormId,
    weapon: FormId,
    first_person: bool,
) -> Option<String> {
    let flags = world::weapon_mods::flags(state, shooter, weapon);
    if shooter == PLAYER_REF && first_person {
        world::weapon_mods::player_model(order, weapon, flags)
    } else {
        world::weapon_mods::model(order, weapon, flags)
    }
}

/// What starting the effects needs.
#[derive(bevy::ecs::system::SystemParam)]
pub struct StartParams<'w, 's> {
    commands: Commands<'w, 's>,
    time: Res<'w, Time>,
    game: Res<'w, GameFiles>,
    state: Res<'w, DialogueState>,
    effects: ResMut<'w, WeaponEffects>,
    wavs: ResMut<'w, Assets<PcmSound>>,
    view: Res<'w, crate::player_camera::PlayerView>,
    model: Res<'w, crate::viewmodel::ViewModel>,
    body: Res<'w, crate::player_body::PlayerBody>,
    cameras: Query<'w, 's, &'static Transform, With<FlyCamera>>,
    rigs: Rigs<'w, 's>,
    globals: Query<'w, 's, &'static GlobalTransform>,
}

/// Plays this frame's shots' sounds and lights their flashes; plays the
/// swings that met no one.
pub fn start_effects(mut p: StartParams) {
    let shots = SHOTS
        .lock()
        .map(|mut q| std::mem::take(&mut *q))
        .unwrap_or_default();
    let swings = SWINGS
        .lock()
        .map(|mut q| std::mem::take(&mut *q))
        .unwrap_or_default();
    if shots.is_empty() && swings.is_empty() {
        return;
    }
    let game = p.game.0.clone();
    let order = &game.order;
    let state = &p.state.0;
    let listener = p
        .cameras
        .single()
        .map(|c| game_point(c.translation))
        .unwrap_or([0.0; 3]);
    let now_ms = (p.time.elapsed_secs_f64() * 1000.0) as u64;
    let first_person = in_first_person(&p.view);
    let mods = FireSoundMods::for_cell(
        order,
        state.player_cell.filter(|_| state.player_world.is_none()),
    );
    for shot in shots {
        let Some(weapon) = world::combat::Weapon::load(order, shot.weapon) else {
            continue;
        };
        let player = shot.shooter == PLAYER_REF;
        let fp = player && first_person;
        let feet = if player {
            state.player_position
        } else {
            state.place(order, shot.shooter).map(|p| p.2)
        };
        // The fire node: on the weapon in hand on screen.
        let model = held_model(order, state, shot.shooter, shot.weapon, fp);
        let node = model.as_deref().and_then(|m| p.effects.node(&game, m));
        let joint = weapon_joint(shot.shooter, fp, &p.model, &p.body, &p.rigs);
        let node_at = joint
            .and_then(|j| p.globals.get(j).ok())
            .zip(node.as_ref())
            .map(|(g, n)| fire_node_world(g, n).w_axis.truncate().to_array());
        let at = shot.from.or(node_at).or(feet).unwrap_or(listener);
        // The sounds (`0083ac30`).
        let sounds = WeaponSounds::load(order, shot.weapon).unwrap_or_default();
        let silenced = player
            && weapon_fx::SILENCER_EFFECTS.iter().any(|&e| {
                world::weapon_mods::has_effect(
                    order,
                    world::weapon_mods::flags(state, shot.shooter, shot.weapon),
                    shot.weapon,
                    e,
                )
            });
        let levels: HashMap<FormId, Option<SoundLevels>> =
            [sounds.set(silenced), sounds.set(false)]
                .into_iter()
                .flatten()
                .flatten()
                .map(|id| (id, p.effects.levels(order, id)))
                .collect();
        let plan =
            weapon_fx::plan_fire_sounds(&sounds, player, silenced, mods, (listener, at), |id| {
                levels.get(&id).copied().flatten()
            });
        if let Some(plan) = plan {
            let d2 = (0..3).map(|k| (at[k] - listener[k]).powi(2)).sum::<f32>();
            // Its entry among the 20 (`busy`: its sounds still playing).
            let busy: Vec<bool> = p
                .effects
                .slot_entities
                .iter()
                .map(|es| es.iter().any(|e| p.commands.get_entity(*e).is_ok()))
                .collect();
            let slot = p
                .effects
                .slots
                .take(shot.shooter, player, d2, now_ms, |i| busy[i]);
            if let Some(slot) = slot {
                p.effects.slot_entities[slot].clear();
                let near = plan.near.map(|s| (s, if s.two_d { "2D" } else { "3D" }));
                for (s, kind) in near.into_iter().chain(plan.far.map(|s| (s, "distant"))) {
                    let offset = match feet {
                        Some(f) if !s.two_d && shot.from.is_none() => {
                            [at[0] - f[0], at[1] - f[1], at[2] - f[2]]
                        }
                        _ => at,
                    };
                    let placed = PlacedSound {
                        shooter: shot.shooter,
                        offset,
                        follows: !s.two_d && shot.from.is_none() && feet.is_some(),
                        distances: s.distances.unwrap_or((0.0, 0.0)),
                        curve: levels
                            .get(&s.sound)
                            .copied()
                            .flatten()
                            .map_or(weapon_fx::DEFAULT_CURVE_MB, |l| l.curve_mb()),
                        static_attenuation: levels
                            .get(&s.sound)
                            .copied()
                            .flatten()
                            .map_or(0, |l| l.static_attenuation),
                        volume: s.volume,
                    };
                    let d = d2.sqrt();
                    let amp = loudness(&placed, if s.two_d { None } else { Some(d) });
                    let name = order
                        .get(s.sound)
                        .and_then(|r| r.editor_id().ok().flatten())
                        .unwrap_or_else(|| s.sound.to_string());
                    println!(
                        "  firing sound {name} ({}) from {} at {d:.0} units: {:.1} dB",
                        kind,
                        shot.shooter,
                        20.0 * amp.max(1e-5).log10()
                    );
                    if let Some(e) = start_sound(
                        &mut p.commands,
                        &game,
                        &mut p.wavs,
                        s.sound,
                        now_ms,
                        amp,
                        1.0,
                    ) {
                        if !s.two_d {
                            p.commands.entity(e).insert(placed);
                        }
                        p.effects.slot_entities[slot].push(e);
                    }
                }
            }
        }
        // The flash (`009c2ff0`, `009bb6d0`).
        let Some(projectile) = weapon_fx::shot_projectile(order, state, shot.shooter, &weapon)
            .and_then(|id| ProjectileEffects::load(order, id))
        else {
            continue;
        };
        let lit = projectile.lights_muzzle_flash()
            && shot.from.is_none()
            && weapon_fx::shooter_lights_flash(
                state.dead.contains(&shot.shooter),
                state.unconscious.contains(&shot.shooter),
            );
        if !lit {
            continue;
        }
        let model = projectile.muzzle_flash_model.clone().unwrap_or_default();
        let key = (shot.shooter, fp);
        let fresh = p
            .effects
            .flashes
            .get(&key)
            .is_none_or(|f| f.projectile != projectile.form_id || f.model != model);
        if fresh {
            if let Some(old) = p.effects.flashes.remove(&key) {
                for piece in old.pieces {
                    if let Ok(mut e) = p.commands.get_entity(piece.entity) {
                        e.despawn();
                    }
                }
            }
            p.effects.flashes.insert(
                key,
                Flash {
                    state: MuzzleFlash::new(projectile.muzzle_flash_duration),
                    projectile: projectile.form_id,
                    model,
                    first_person: fp,
                    pieces: Vec::new(),
                    node,
                },
            );
        }
        if let Some(f) = p.effects.flashes.get_mut(&key) {
            f.node = node;
            let was = f.state.shown;
            f.state.fire();
            if f.state.shown && !was {
                println!(
                    "  muzzle flash {} on {} at {}",
                    f.model,
                    shot.shooter,
                    node_at.map_or("(no fire node)".to_string(), |a| format!(
                        "{:.0},{:.0},{:.0}",
                        a[0], a[1], a[2]
                    ))
                );
            }
        }
    }
    // Swings that met no one (`00899200`): the weapon's `TNAM`, at the
    // attacker, with the sound's own distances.
    for (attacker, weapon) in swings {
        let Some(swing) = WeaponSounds::load(order, weapon).and_then(|s| s.swing) else {
            continue;
        };
        let Some(levels) = p.effects.levels(order, swing) else {
            continue;
        };
        let feet = if attacker == PLAYER_REF {
            state.player_position
        } else {
            state.place(order, attacker).map(|p| p.2)
        };
        let at = feet.unwrap_or(listener);
        let placed = PlacedSound {
            shooter: attacker,
            offset: [0.0; 3],
            follows: feet.is_some(),
            distances: levels.distances(),
            curve: levels.curve_mb(),
            static_attenuation: levels.static_attenuation,
            volume: 1.0,
        };
        let d = (0..3)
            .map(|k| (at[k] - listener[k]).powi(2))
            .sum::<f32>()
            .sqrt();
        let amp = loudness(&placed, Some(d));
        if let Some(e) = start_sound(&mut p.commands, &game, &mut p.wavs, swing, now_ms, amp, 1.0) {
            p.commands.entity(e).insert(placed);
        }
    }
}

/// Starts a sound record at a fixed point (an impact's: `009c20e0`,
/// `0088e1e0` play them at the point with flags 0x4102 and the sound's own
/// distances), as loud as those distances make it from the listener now,
/// and kept so every frame after (`attenuate`). Its loudness (dB) when it
/// starts.
pub(crate) fn play_at(
    commands: &mut Commands,
    game: &cellview::Game,
    wavs: &mut Assets<PcmSound>,
    sound: FormId,
    (at, listener): ([f32; 3], [f32; 3]),
    pick: u64,
) -> Option<f32> {
    play_at_with(commands, game, wavs, sound, (at, listener), pick, None, 1.0)
}

/// [`play_at`] with the static attenuation (hundredths of a decibel) set
/// in place of the sound's own (`None`: its own) and its frequency ×
/// `speed`, as `ImpactMixer::PlayCollisionSound` (`00837550`) sets them on
/// a contact's sounds (`00ad89b0`, `00ad8a90`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn play_at_with(
    commands: &mut Commands,
    game: &cellview::Game,
    wavs: &mut Assets<PcmSound>,
    sound: FormId,
    (at, listener): ([f32; 3], [f32; 3]),
    pick: u64,
    static_attenuation: Option<i16>,
    speed: f32,
) -> Option<f32> {
    let levels = SoundLevels::load(&game.order, sound)?;
    let placed = PlacedSound {
        shooter: FormId(0),
        offset: at,
        follows: false,
        distances: levels.distances(),
        curve: levels.curve_mb(),
        static_attenuation: static_attenuation.unwrap_or(levels.static_attenuation),
        volume: 1.0,
    };
    let d = (0..3)
        .map(|k| (at[k] - listener[k]).powi(2))
        .sum::<f32>()
        .sqrt();
    let amp = loudness(&placed, Some(d));
    let e = start_sound(commands, game, wavs, sound, pick, amp, speed)?;
    commands.entity(e).insert(placed);
    Some(20.0 * amp.max(1e-5).log10())
}

/// How loud a placed sound is `d` units from the listener (`None`: 2D).
fn loudness(s: &PlacedSound, d: Option<f32>) -> f32 {
    let (min, max) = weapon_fx::buffer_distances(s.distances.0, s.distances.1);
    let distance = d.map_or(0, |d| weapon_fx::distance_attenuation(d, min, max, s.curve));
    weapon_fx::amplitude(s.volume, s.static_attenuation, distance)
}

/// Starts a sound record playing at an amplitude.
fn start_sound(
    commands: &mut Commands,
    game: &cellview::Game,
    wavs: &mut Assets<PcmSound>,
    id: FormId,
    pick: u64,
    amplitude: f32,
    speed: f32,
) -> Option<Entity> {
    let sound = world::sound::Sound::load(&game.order, id)?;
    let settings = PlaybackSettings {
        volume: Volume::Linear(amplitude),
        speed,
        ..PlaybackSettings::DESPAWN
    };
    crate::sounds::play_with(commands, game, wavs, &sound, pick, settings)
}

/// Keeps placed sounds as loud as the listener's distance says
/// (`00aed990` every frame), following their shooter.
pub fn attenuate(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut sounds: Query<(&PlacedSound, &mut AudioSink)>,
) {
    let Ok(camera) = cameras.single() else {
        return;
    };
    let listener = game_point(camera.translation);
    let order = &game.0.order;
    let state = &state.0;
    for (s, mut sink) in &mut sounds {
        let at = if s.follows {
            let feet = if s.shooter == PLAYER_REF {
                state.player_position
            } else {
                state.place(order, s.shooter).map(|p| p.2)
            };
            feet.map_or(s.offset, |f| {
                [f[0] + s.offset[0], f[1] + s.offset[1], f[2] + s.offset[2]]
            })
        } else {
            s.offset
        };
        let d = (0..3)
            .map(|k| (at[k] - listener[k]).powi(2))
            .sum::<f32>()
            .sqrt();
        let amp = loudness(s, Some(d));
        if (sink.volume().to_linear() - amp).abs() > 1e-4 {
            sink.set_volume(Volume::Linear(amp));
        }
    }
}

/// Runs the flashes' clocks (`009bb080`), puts their models on screen when
/// first lit, and shows or hides them.
pub fn update_flashes(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut effects: ResMut<WeaponEffects>,
    mut spawner: Spawner,
    mut visibility: Query<&mut Visibility>,
) {
    let dt = time.delta_secs();
    let game = game.0.clone();
    let lighting = spawner.place_lighting.get();
    let effects = &mut *effects;
    let mut gone = Vec::new();
    for (key, flash) in effects.flashes.iter_mut() {
        flash.state.update(dt);
        // A place change took its pieces away: made again when next lit.
        if flash
            .pieces
            .iter()
            .any(|p| visibility.get(p.entity).is_err())
        {
            flash.pieces.clear();
        }
        if flash.pieces.is_empty() && flash.state.shown {
            let scene = effects
                .scenes
                .entry(flash.model.to_ascii_lowercase())
                .or_insert_with(|| flash_scene(&game, flash.projectile, &flash.model))
                .clone();
            let Some(scene) = scene else {
                gone.push(*key);
                continue;
            };
            let Some(lighting) = lighting else {
                continue;
            };
            let entities = spawner.spawn_with(&scene, Some(lighting));
            let mut pieces = Vec::new();
            let mut spawned = entities.into_iter();
            for draw in scene.draws.iter().filter(|d| d.actor.is_none()) {
                let Some(entity) = spawned.next() else {
                    break;
                };
                let data = &scene.meshes[draw.mesh];
                pieces.push(FlashPiece {
                    entity,
                    local: Mat4::from_cols_array(&draw.transform),
                    billboard: data.billboard,
                    center: Vec3::from(crate::sort_center(data).unwrap_or([0.0; 3])),
                });
                let mut e = spawner.commands.entity(entity);
                e.remove::<(crate::Moving, crate::Facing)>()
                    .insert((NoFrustumCulling, Visibility::Hidden));
                if flash.first_person {
                    e.insert(RenderLayers::layer(crate::viewmodel::FIRST_PERSON_LAYER));
                }
            }
            // Particles in the model aren't placed with it: left out.
            for rest in spawned {
                if let Ok(mut e) = spawner.commands.get_entity(rest) {
                    e.despawn();
                }
            }
            flash.pieces = pieces;
        }
        let want = if flash.state.shown {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        for piece in &flash.pieces {
            if let Ok(mut v) = visibility.get_mut(piece.entity) {
                if *v != want {
                    *v = want;
                }
            }
        }
    }
    for key in gone {
        effects.flashes.remove(&key);
    }
}

/// A muzzle flash model on its own at the origin, ready to draw.
fn flash_scene(
    game: &cellview::Game,
    projectile: FormId,
    model: &str,
) -> Option<Arc<cellview::ViewerScene>> {
    let placement = world::Placement {
        form_id: projectile,
        record_type: esm::FourCC::new(b"REFR"),
        editor_id: None,
        base: projectile,
        base_type: esm::FourCC::new(b"PROJ"),
        base_editor_id: None,
        position: [0.0; 3],
        rotation: [0.0; 3],
        scale: 1.0,
        model: Some(model.to_string()),
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
    };
    let scene = game.made_scene(vec![placement]);
    (!scene.draws.is_empty()).then(|| Arc::new(scene))
}

/// Puts the shown flashes at their weapon's fire node, once everything has
/// moved this frame (`009bb240`), their billboards facing the camera.
pub fn place_flashes(
    time: Res<Time>,
    mut effects: ResMut<WeaponEffects>,
    model: Res<crate::viewmodel::ViewModel>,
    body: Res<crate::player_body::PlayerBody>,
    rigs: Rigs,
    cameras: Query<&GlobalTransform, With<FlyCamera>>,
    mut placed: Query<(&mut Transform, &mut GlobalTransform), Without<FlyCamera>>,
) {
    let mut shown = None;
    let camera = cameras.single().ok().map(|c| c.compute_transform());
    let game = |v: Vec3| [v.x, -v.z, v.y];
    let eye_axes = camera.map(|c| {
        (
            game(c.translation / space::METERS_PER_UNIT),
            [
                game(c.rotation * Vec3::X),
                game(c.rotation * Vec3::Y),
                game(c.rotation * Vec3::Z),
            ],
        )
    });
    for ((shooter, fp), flash) in effects.flashes.iter() {
        if !flash.state.shown || flash.pieces.is_empty() {
            continue;
        }
        let Some(node) = flash.node.as_ref() else {
            continue;
        };
        let Some(joint) = weapon_joint(*shooter, *fp, &model, &body, &rigs) else {
            continue;
        };
        let Ok((_, joint_global)) = placed.get(joint).map(|(t, g)| (*t, *g)) else {
            continue;
        };
        let world = fire_node_world(&joint_global, node);
        for piece in &flash.pieces {
            let draw = (world * piece.local).to_cols_array();
            let base = Mat4::from_cols_array(&space::matrix(&draw));
            let turned = match (&piece.billboard, eye_axes) {
                (Some(b), Some((eye, axes))) => {
                    Mat4::from_cols_array(&cellview::billboard_matrix(b, &draw, eye, axes))
                }
                _ => Mat4::IDENTITY,
            };
            let m = base * turned * Mat4::from_translation(piece.center);
            if let Ok((mut t, mut g)) = placed.get_mut(piece.entity) {
                *t = Transform::from_matrix(m);
                *g = GlobalTransform::from(m);
            }
        }
        // For `NV_SHOT_ON_FLASH`: one still lit next frame, and (someone
        // else's) in front of the camera.
        let ahead = camera.is_some_and(|c| {
            let at = Vec3::from(space::point(world.w_axis.truncate().to_array()));
            (at - c.translation)
                .normalize_or_zero()
                .dot(c.forward().as_vec3())
                > 0.8
        });
        if flash.state.left > time.delta_secs() * 1.5 && (*shooter == PLAYER_REF || ahead) {
            shown = Some(*shooter == PLAYER_REF);
        }
    }
    effects.shown = shown;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sound(distances: (f32, f32)) -> PlacedSound {
        PlacedSound {
            shooter: FormId(0x10),
            offset: [0.0; 3],
            follows: false,
            distances,
            curve: [0, 602, 1398, 2602, 5000],
            static_attenuation: 1057,
            volume: 1.0,
        }
    }

    /// A 3D firing sound fades along its curve; a distant one whose 3D
    /// sound didn't start (distances 0, 0) carries at its own level; a 2D
    /// one has no distance at all.
    #[test]
    fn placed_sounds_fade_as_the_game_works_them_out() {
        let near = sound((255.0, 2400.0));
        let at = |d| 20.0 * loudness(&near, Some(d)).log10();
        assert!((at(100.0) - -10.57).abs() < 0.01);
        assert!((at(791.25) - -16.59).abs() < 0.01);
        assert!(at(2500.0) <= -99.9);
        let far = sound((0.0, 0.0));
        assert!((20.0 * loudness(&far, Some(3000.0)).log10() - -10.57).abs() < 0.01);
        assert_eq!(loudness(&near, None), loudness(&near, Some(0.0)));
    }
}

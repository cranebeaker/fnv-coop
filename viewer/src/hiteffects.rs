//! What hits sound like (`world::impacts`, read from the game's code): the
//! player's shots and blows (`combat`) and people's and creatures'
//! (`fighting`) report each hit here, and this plays it as the game's hit
//! handler does.
//!
//! - **On someone** (`0088e1e0`): the blow's impact sounds, each only when
//!   the camera is nearer than that sound's largest distance; their hurt
//!   line ("Hit", `0089a760`) when the hit took more than
//!   `fCombatSpeakHitThreshold` of their health or by chance, no sooner
//!   than the shared cooldown (`fDialogHitSoundCooldownMin`–`Max`, 2–4 s)
//!   and 1.5 s after any combat line (`009839b0`); on death (`0089d900`)
//!   their death cry: a creature's "death" sound, else their "Death"
//!   line. The player hit gets the `GetHit` image space modifier at
//!   strength `fGetHitPainMult` (1.5).
//!   Where the blow met their body: blood ([`blood`], `0088e8d0`), the
//!   blood impact's model and a spatter decal on the wall behind.
//! - **On the world** (shots, `009c20e0`): the weapon's impact for the
//!   struck surface's Havok material (kept with each collision triangle;
//!   land without one counts as dirt, the game's default): its decal, its
//!   effect model and its two sounds.
//!
//! Impact sounds play at the point with their own distances
//! (`weapon_fx::play_at`, faded as the listener moves); effect models and
//! decals are drawn by `impact_fx`. Not done: decals on skin, blood on the
//! player's screen, sounds placed left or right.

use std::collections::HashMap;

use bevy::audio::AudioPlayer;
use bevy::prelude::*;
use esm::FormId;
use world::dialogue::{Speaker, PLAYER_REF};
use world::impacts::{self, CombatVoice, DeathCry, Impact, Material};

use bevy::render::primitives::Frustum;
use cellview::space;
use world::decals::DecalRolls;

use crate::dialogue::DialogueState;
use crate::impact_fx::{decal_targets, DecalRequest, DecalTargets, EffectRequest, ImpactRequests};
use crate::sounds::SoundRequests;
use crate::walk::game_point;
use crate::{FlyCamera, GameFiles};

/// One hit, as `combat` and `fighting` report it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitReport {
    pub attacker: FormId,
    /// Who was hit; `None`: the world (a wall, the floor).
    pub target: Option<FormId>,
    /// The hit's weapon (`None`: fists or a creature's own attack).
    pub weapon: Option<FormId>,
    /// Where it struck (game units).
    pub point: [f32; 3],
    /// On the world: the struck surface's Havok material, if it has one.
    pub havok: Option<u32>,
    /// On the world: the struck surface's normal, facing the shot, and the
    /// collision triangle it struck.
    pub normal: Option<[f32; 3]>,
    pub triangle: Option<u32>,
    /// Which way the shot or blow went (unit; zero when not known).
    pub direction: [f32; 3],
    /// On someone: `point` is where it met their body (not just where
    /// they stand), so blood can show there.
    pub on_body: bool,
    /// On someone: the body part it struck, if known.
    pub part: Option<u8>,
    /// On someone: the health damage, and whether it killed them.
    pub damage: f32,
    pub killed: bool,
}

/// Hits reported this frame.
#[derive(Resource, Default)]
pub struct HitReports(pub Vec<HitReport>);

impl HitReports {
    /// A shot from `eye` along `dir` striking the collider's triangle `tri`
    /// `d` units away.
    pub fn shot_on_world(
        &mut self,
        collider: &physics::Collider,
        (eye, dir): ([f32; 3], [f32; 3]),
        (d, tri): (f32, u32),
        attacker: FormId,
        weapon: FormId,
    ) {
        // A body Havok moves takes the shot's push (`clutter`).
        crate::clutter::shot(weapon, attacker, (eye, dir), d, collider.owner(tri));
        self.0.push(HitReport {
            attacker,
            target: None,
            weapon: Some(weapon),
            point: [0, 1, 2].map(|k| eye[k] + dir[k] * d),
            havok: collider.material(tri),
            normal: crate::impact_fx::facing_normal(collider.triangle(tri), dir),
            triangle: Some(tri),
            direction: dir,
            on_body: false,
            part: None,
            damage: 0.0,
            killed: false,
        });
    }
}

/// The combat lines' clocks, and the impact records read.
#[derive(Resource, Default)]
pub struct HitEffects {
    voice: CombatVoice,
    impacts: HashMap<FormId, Option<Impact>>,
}

pub struct HitEffectsPlugin;

impl Plugin for HitEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HitReports>()
            .init_resource::<HitEffects>()
            .add_systems(
                Update,
                // In the AI work (stage 6, `crate::frame_order`), after the
                // player's attack (stage 2) by the stages' order. Placed for
                // order only, right after the actors' movement pass, whose
                // projectiles strike (`crate::frame_order::AiSet`), outside its
                // gate: it also plays the player's hits (its exact place isn't
                // traced).
                play_hits
                    // Kept: after people's moves.
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
            );
    }
}

fn distance(a: [f32; 3], b: [f32; 3]) -> f32 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2) + (a[2] - b[2]).powi(2)).sqrt()
}

/// 0..1 from the game state's dice.
fn unit(state: &mut world::scripting::GameState) -> f32 {
    (state.roll() % 1_000_000) as f32 / 1_000_000.0
}

/// Everything playing a hit needs.
#[derive(bevy::ecs::system::SystemParam)]
pub struct HitParams<'w, 's> {
    commands: Commands<'w, 's>,
    time: Res<'w, Time>,
    game: Res<'w, GameFiles>,
    state: ResMut<'w, DialogueState>,
    reports: ResMut<'w, HitReports>,
    effects: ResMut<'w, HitEffects>,
    sounds: ResMut<'w, SoundRequests>,
    screen: ResMut<'w, crate::effects::Effects>,
    audio: ResMut<'w, Assets<crate::sounds::PcmSound>>,
    cameras: Query<'w, 's, &'static Transform, With<FlyCamera>>,
    frusta: Query<'w, 's, &'static Frustum, With<FlyCamera>>,
    collision: Res<'w, crate::walk::CellCollision>,
    requests: ResMut<'w, ImpactRequests>,
}

/// Whether a sphere of `radius` around a point (game units) is in the
/// camera's view (`004b61d0`, `004b5ff0`: not wholly outside any of the
/// six planes).
fn in_view(frusta: &Query<&Frustum, With<FlyCamera>>, point: [f32; 3], radius: f32) -> bool {
    frusta.single().is_ok_and(|f| {
        f.intersects_sphere(
            &bevy::render::primitives::Sphere {
                center: Vec3::from(space::point(point)).into(),
                radius: radius * space::METERS_PER_UNIT,
            },
            true,
        )
    })
}

/// Plays this frame's hits.
pub fn play_hits(mut p: HitParams) {
    if p.reports.0.is_empty() {
        return;
    }
    let reports = std::mem::take(&mut p.reports.0);
    let game = p.game.0.clone();
    let now = p.time.elapsed_secs();
    let now_ms = (p.time.elapsed_secs_f64() * 1000.0) as u64;
    let eye = p
        .cameras
        .single()
        .map(|c| game_point(c.translation))
        .unwrap_or([0.0; 3]);
    for r in reports {
        match r.target {
            Some(target) => on_someone(&mut p, &game, &r, target, (eye, now, now_ms)),
            None => on_world(&mut p, &game, &r, eye),
        }
    }
}

/// The impact record, read once.
fn impact(effects: &mut HitEffects, order: &esm::LoadOrder, id: FormId) -> Option<Impact> {
    effects
        .impacts
        .entry(id)
        .or_insert_with(|| Impact::load(order, id))
        .clone()
}

/// The name of a record, for the log.
fn name_of(order: &esm::LoadOrder, id: FormId) -> String {
    order
        .get(id)
        .and_then(|rr| rr.editor_id().ok().flatten())
        .unwrap_or_else(|| id.to_string())
}

/// Plays a hit's sound at a point (`weapon_fx::play_at`) and says so.
fn sound_at(p: &mut HitParams, game: &cellview::Game, s: FormId, at: [f32; 3], eye: [f32; 3]) {
    let pick = p.state.0.roll();
    if let Some(db) =
        crate::weapon_fx::play_at(&mut p.commands, game, &mut p.audio, s, (at, eye), pick)
    {
        println!(
            "  impact sound {} at {:.0} units: {db:.1} dB",
            name_of(&game.order, s),
            distance(at, eye)
        );
    }
}

/// A shot striking the world (`009c20e0`): the weapon's impact for the
/// struck shape's material. Its decal on what's there of that material
/// (`world::decals`); its effect model at the point when within
/// `fGunParticleCameraDistance` of the camera and in view (a sphere of
/// 32), along its orientation (`world::impacts::effect_axis`), for its
/// duration; its two sounds at the point (flags 0x4102), at any distance.
/// (The game also skips the model while `[011dea2a]` is set, a state not
/// kept here.)
fn on_world(p: &mut HitParams, game: &cellview::Game, r: &HitReport, eye: [f32; 3]) {
    let order = &game.order;
    let Some(weapon) = r.weapon else {
        return;
    };
    // Land and anything added without a material: the land's default,
    // dirt (`00457880`; the land texture's own material isn't looked up).
    let material = r
        .havok
        .map_or(Material::Dirt, |h| Material::from_havok(h & 0x1f));
    let Some(i) = impacts::surface_impact(order, weapon, material)
        .and_then(|id| impact(&mut p.effects, order, id))
    else {
        return;
    };
    println!(
        "  the shot strikes {} ({})",
        material.name(),
        i.editor_id.as_deref().unwrap_or("")
    );
    let normal = r.normal.unwrap_or(r.direction.map(|c| -c));
    // The decal: its size rolled once for the shot, then its turn and
    // picture (`009c20e0`).
    if let (Some(decal), Some(_)) = (i.decal, i.texture_set) {
        let state = &mut p.state.0;
        let rolls = DecalRolls {
            size: unit(state),
            turn: unit(state),
            picture: unit(state),
        };
        let size = decal.size(rolls.size, false);
        let reach = (size * size * 0.5 + decal.depth * decal.depth).sqrt();
        let targets = decal_targets(order, &p.collision.0, r.point, reach, material);
        if targets != DecalTargets::default() {
            p.requests.decals.push(DecalRequest {
                impact: i.form_id,
                point: r.point,
                normal,
                rolls,
                depth: None,
                targets,
            });
        }
    }
    if let Some(model) = &i.model {
        if distance(eye, r.point) <= impacts::effect_distance(order)
            && in_view(&p.frusta, r.point, 32.0)
        {
            let roll = unit(&mut p.state.0);
            p.requests.effects.push(EffectRequest {
                model: model.clone(),
                point: r.point,
                axis: impacts::effect_axis(i.orientation, normal, r.direction),
                roll,
                given: i.duration,
            });
        }
    }
    for s in i.sounds().collect::<Vec<_>>() {
        sound_at(p, game, s, r.point, eye);
    }
}

/// Blood where a hit met someone's body (`0088e8d0`, when hits on them show
/// blood and did damage): shown within `fGunParticleCameraDistance` of the
/// camera, or beyond it out of view. The blood impact's model at the
/// point, its Z axis against the spray (`world::impacts::spray_direction`:
/// twice the hit's direction plus U(−0.8, 0.8) on each axis), for its
/// animation or a second; then, with chance `fCombatEnvironmentBloodChance`
/// (exe 0.75), the body part's own impact's decal on the first thing 512
/// units along the spray tipped down (`world::impacts::spatter_direction`)
/// that takes decals, 48 deep. (Decals on the body itself, and Bloody
/// Mess's ×1.5, which waits on actor value 55 that nothing raises here,
/// aren't done.)
fn blood(
    p: &mut HitParams,
    game: &cellview::Game,
    r: &HitReport,
    effects: &impacts::ActorHitEffects,
    eye: [f32; 3],
) {
    let order = &game.order;
    let near = distance(eye, r.point) <= impacts::effect_distance(order);
    if !near && in_view(&p.frusta, r.point, 32.0) {
        return;
    }
    let state = &mut p.state.0;
    let jitter = [0; 3].map(|_| -0.8 + 1.6 * unit(state));
    let spray = impacts::spray_direction(r.direction, jitter);
    if let Some(i) = effects
        .blood
        .and_then(|id| impact(&mut p.effects, order, id))
    {
        if let Some(model) = &i.model {
            let roll = unit(&mut p.state.0);
            p.requests.effects.push(EffectRequest {
                model: model.clone(),
                point: r.point,
                axis: spray.map(|c| -c),
                roll,
                given: 1.0,
            });
        }
    }
    let Some(spatter) = effects.spatter else {
        return;
    };
    let chance =
        world::scripting::game_setting(order, "fCombatEnvironmentBloodChance").unwrap_or(0.75);
    if unit(&mut p.state.0) >= chance {
        return;
    }
    let dir = impacts::spatter_direction(spray);
    let collider = &p.collision.0;
    let Some((d, t)) = collider.raycast(r.point, dir, impacts::SPATTER_REACH) else {
        return;
    };
    let Some(targets) = crate::impact_fx::struck_target(order, collider, t) else {
        return;
    };
    let Some(normal) = crate::impact_fx::facing_normal(collider.triangle(t), dir) else {
        return;
    };
    let state = &mut p.state.0;
    let rolls = DecalRolls {
        size: unit(state),
        turn: unit(state),
        picture: unit(state),
    };
    p.requests.decals.push(DecalRequest {
        impact: spatter,
        point: [0, 1, 2].map(|k| r.point[k] + dir[k] * d),
        normal,
        rolls,
        depth: Some(world::decals::SPATTER_DEPTH),
        targets,
    });
}

/// A hit on a person or creature (`0089a760` and the functions it calls).
fn on_someone(
    p: &mut HitParams,
    game: &cellview::Game,
    r: &HitReport,
    target: FormId,
    (eye, now, now_ms): ([f32; 3], f32, u64),
) {
    let order = &game.order;
    let held = world::combat::weapon_in_hand(order, &p.state.0, r.attacker).map(|w| w.form_id);
    let gore_off = game
        .settings
        .float("General", "bDisableAllGore")
        .is_some_and(|v| v != 0.0);
    let effects = {
        let state = &mut p.state.0;
        let mut dice = state.dice.max(1);
        let mut roll = || {
            dice ^= dice << 13;
            dice ^= dice >> 7;
            dice ^= dice << 17;
            dice
        };
        let e = impacts::actor_hit(
            order, state, r.attacker, target, r.weapon, held, r.part, gore_off, &mut roll,
        );
        state.dice = dice.max(1);
        e
    };
    // Sounds, each heard within its own distance (the player hit hears
    // them all: the game measures from the player then).
    // Played at the point (flags 0x4102); the player hit hears them where
    // the player is.
    let at = if target == PLAYER_REF { eye } else { r.point };
    let mut heard = Vec::new();
    for s in &effects.sounds {
        let reach = world::sound::Sound::load(order, *s).map_or(0.0, |s| s.max_distance);
        if target == PLAYER_REF || distance(eye, r.point) < reach {
            heard.push(name_of(order, *s));
            sound_at(p, game, *s, at, eye);
        }
    }
    if !heard.is_empty() {
        println!(
            "  {target} hit ({}): {}",
            effects.material.name(),
            heard.join(", ")
        );
    }
    // Blood (the player's own is the screen's: not done here).
    if r.on_body && r.damage > 0.0 && target != PLAYER_REF {
        blood(p, game, r, &effects, eye);
    }
    // The player hit by someone: the hit modifier at its strength.
    if target == PLAYER_REF && r.attacker != PLAYER_REF {
        let (modifier, strength) = impacts::get_hit_modifier(order);
        p.screen.instances.push((modifier, now, strength));
    }
    // What they say or cry out.
    let state = &mut p.state.0;
    if r.killed {
        let mut roll = || state.roll();
        match impacts::death_cry(order, target, &mut roll) {
            DeathCry::Sound(s) => p.sounds.0.push(s),
            DeathCry::Line => {
                if target != PLAYER_REF && p.effects.voice.line_allowed(now_ms) {
                    say(p, game, target, impacts::DEATH_TOPIC);
                }
            }
        }
        return;
    }
    let health = world::combat::health(order, state, target).unwrap_or(0.0) as f32;
    let t = unit(state);
    let wants = impacts::says_hurt_line(order, r.damage, health, false, t);
    let (lo, hi) = (
        game.settings
            .float("Audio", "fDialogHitSoundCooldownMin")
            .unwrap_or(2.0),
        game.settings
            .float("Audio", "fDialogHitSoundCooldownMax")
            .unwrap_or(4.0),
    );
    let mut pick = |a: u64, b: u64| a + state.roll() % (b - a + 1);
    let allowed = p.effects.voice.hurt_allowed(now_ms, lo, hi, &mut pick);
    if wants && allowed && target != PLAYER_REF && p.effects.voice.line_allowed(now_ms) {
        say(p, game, target, impacts::HIT_TOPIC);
    }
}

/// Someone says a combat topic's line (`009839b0`): the first their
/// conditions allow (`world::dialogue::pick`), in their voice, their face
/// moving with it.
fn say(p: &mut HitParams, game: &cellview::Game, who: FormId, topic: FormId) {
    let order = &game.order;
    let Some(base) = world::scripting::base_of(order, who) else {
        return;
    };
    let Some(speaker) = Speaker::load(order, who, base) else {
        return;
    };
    let Some(info) = world::dialogue::pick(order, topic, &speaker, &p.state.0) else {
        return;
    };
    let Some(response) = info.responses.first() else {
        return;
    };
    println!("  {who} says \"{}\"", response.text);
    let Some(path) = speaker
        .voice
        .and_then(|v| world::dialogue::voice_path(order, &info, response, v))
    else {
        return;
    };
    let Some(bytes) = game.assets.read(&path).ok().flatten() else {
        return;
    };
    let Some(source) = crate::sounds::voice_handle(&path, &bytes, &mut p.audio) else {
        return;
    };
    let (settings, voice) = crate::faces::voice_playback(game, &path, who);
    p.commands.spawn((AudioPlayer(source), settings, voice));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_shot_on_the_world_keeps_the_struck_triangles_material() {
        let mut c = physics::Collider::new();
        c.add_with_material(
            &[[0.0, -50.0, -50.0], [0.0, 50.0, -50.0], [0.0, 0.0, 50.0]],
            &[[0, 1, 2]],
            9,
        );
        let (d, tri) = c
            .raycast([-100.0, 0.0, 0.0], [1.0, 0.0, 0.0], 500.0)
            .unwrap();
        let mut hits = HitReports::default();
        hits.shot_on_world(
            &c,
            ([-100.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
            (d, tri),
            PLAYER_REF,
            FormId(0x123),
        );
        let r = hits.0[0];
        assert_eq!(r.havok, Some(9));
        assert!((r.point[0]).abs() < 1e-3);
        assert_eq!((r.target, r.weapon), (None, Some(FormId(0x123))));
    }
}

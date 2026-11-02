//! Shots whose projectile flies (`world::projectiles`, `docs/ENERGY_WEAPONS.md`):
//! the plasma weapons' bolts, missiles without the "hitscan" flag, leave
//! the muzzle at their speed (the plasma pistol's 7500 units a second) and
//! strike the first body, scripted object or wall on each frame's stretch
//! of their way (`MissileProjectile::UpdateProjectile`, `009b8030`), the
//! shooter passed by; past their range or `fArrowAgeMax` they're gone.
//! Beams (the lasers) and bullets strike at once, as before (`combat`,
//! `fighting`).
//!
//! The player's attack (`combat`) and people's shots (`fighting`) queue a
//! bolt here for each projectile instead of casting along the range. What
//! a bolt strikes takes the hit as the instant shots do (the hit path:
//! damage, criticals and their effects, `OnHit`, `OnDeath`), and is
//! reported to `hiteffects`. Not here: the bolt's model, light and trail
//! (the projectile's `MODL`, muzzle flash; the effects batch), the
//! projectile's own collision shape (it's a point here), people dodging
//! it, and the shooter's Turbo slowing time for it.

use std::sync::Mutex;

use bevy::prelude::*;
use esm::FormId;
use world::combat::Weapon;
use world::dialogue::PLAYER_REF;
use world::projectiles::{Delivery, Missile};
use world::scripting::Runner;

use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::dialogue::{DialogueState, Talkers};
use crate::scripts::{CellScripts, Scripts};
use crate::walk::CellCollision;
use crate::GameFiles;

/// A bolt in flight: who fired it, the share of the weapon's damage it
/// carries, and its way.
#[derive(Debug, Clone)]
pub(crate) struct Bolt {
    pub shooter: FormId,
    pub pellet: Weapon,
    pub projectile: world::explosions::ProjectileRecord,
    pub missile: Missile,
}

impl Bolt {
    /// A projectile that explodes on impact (the plasma caster's: flag
    /// 0x2 with an explosion, no alt. trigger) goes off where it struck
    /// (`009c3190`; `explosives`).
    fn impact(&self, at: [f32; 3]) {
        let p = &self.projectile;
        if p.explodes() && !p.alt_trigger() {
            crate::explosives::detonate(self.shooter, self.pellet.clone(), p.clone(), at);
        }
    }
}

/// Bolts fired this frame: the attacks are in systems that can't take
/// more parameters, so they queue here (as `explosives` does).
static QUEUE: Mutex<Vec<Bolt>> = Mutex::new(Vec::new());

/// Whether a weapon's shots fly (its projectile is delivered by flight,
/// `world::projectiles::delivery`).
pub(crate) fn flies(order: &esm::LoadOrder, weapon: &Weapon) -> bool {
    weapon
        .projectile
        .and_then(|p| world::explosions::ProjectileRecord::load(order, p))
        // Outside V.A.T.S. (its playback shoots in `vats`, which keeps
        // casting at once: bullets flying there aren't done).
        .is_some_and(|p| world::projectiles::delivery(&p, false) == Delivery::Flies)
}

/// Fires a bolt of `pellet`'s projectile from `origin` along `dir`
/// (unit), at its launch speed (`world::explosions::launch_speed`) as far
/// as its range.
pub(crate) fn fire(
    order: &esm::LoadOrder,
    state: &world::scripting::GameState,
    shooter: FormId,
    pellet: &Weapon,
    (origin, dir): ([f32; 3], [f32; 3]),
) {
    let Some(record) = pellet
        .projectile
        .and_then(|p| world::explosions::ProjectileRecord::load(order, p))
    else {
        return;
    };
    let speed = world::explosions::launch_speed(order, state, shooter, pellet, &record, 1.0);
    let range = if record.range > 0.0 {
        record.range
    } else {
        crate::combat::SHOT_RANGE
    };
    if let Ok(mut q) = QUEUE.lock() {
        q.push(Bolt {
            shooter,
            pellet: pellet.clone(),
            missile: Missile::launch(origin, dir, speed, range)
                .with_gravity(world::projectiles::missile_gravity(&record)),
            projectile: record,
        });
    }
}

/// What's in flight.
#[derive(Resource, Default)]
pub struct Bolts(Vec<Bolt>);

pub struct BoltsPlugin;

impl Plugin for BoltsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bolts>().add_systems(
            Update,
            (
                // The process list's projectiles move in the actors' movement
                // pass (`ProcessLists::UpdateActorsMovement`, `0096db30`: its
                // non-actor objects get `009bec10`, `Projectile`'s slot +0x2f8),
                // on the AI threads (`crate::frame_order::AiSet`), under its
                // gate: bolts hold still in menu mode. After the player's attack
                // (stage 2) by the stages' order.
                fly_bolts
                    // Kept: after people's shots (same pass), before the hits'
                    // effects.
                    .after(crate::fighting::resolve_shots)
                    .before(crate::hiteffects::play_hits)
                    .in_set(crate::frame_order::AiSet::Call(
                        crate::frame_order::ACTORS_MOVEMENT,
                    )),
                // Placed for order only (its place isn't traced): culled
                // corpses hidden after the pass, before the next call.
                hide_culled_bodies
                    .in_set(crate::frame_order::FrameSet::Stage(
                        world::frame::Stage::AiStart,
                    ))
                    .after(crate::frame_order::AiSet::Call(
                        crate::frame_order::ACTORS_MOVEMENT,
                    ))
                    .before(crate::frame_order::AiSet::next(
                        crate::frame_order::ACTORS_MOVEMENT,
                    )),
            ),
        );
    }
}

/// Each bolt goes its frame's way (`009bf300`) and strikes the first
/// thing met on it (`fighting`'s line test: bodies' capsules, scripted
/// objects, walls, the player's bounds for others' bolts); then it's
/// spent past its range or age (`009b8030`).
#[allow(clippy::too_many_arguments)]
pub fn fly_bolts(
    time: Res<Time>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mut dialogue: ResMut<DialogueState>,
    mut caches: ResMut<crate::combat::PlayerAttack>,
    (talkers, cell_scripts, collision): (Res<Talkers>, Res<CellScripts>, Res<CellCollision>),
    mut hits: ResMut<crate::hiteffects::HitReports>,
    mut messages: ResMut<crate::hud::HudMessages>,
    mut bolts: ResMut<Bolts>,
    rigs: Query<(&Walker, &ActorRig)>,
) {
    if let Ok(mut q) = QUEUE.lock() {
        bolts.0.append(&mut q);
    }
    if bolts.0.is_empty() {
        return;
    }
    let order = &game.0.order;
    let state = &mut dialogue.0;
    let now = time.elapsed_secs();
    let dt = time.delta_secs();
    let age_max = world::projectiles::age_max(order);
    let mut flying = Vec::with_capacity(bolts.0.len());
    for mut bolt in std::mem::take(&mut bolts.0) {
        // This frame's way: along its heading and, with gravity, falling
        // (`world::projectiles::Missile::stretch`).
        let stretch = bolt.missile.stretch(dt);
        let length = stretch.length;
        let (from, dir, out) = (
            bolt.missile.position,
            stretch.direction,
            bolt.missile.travelled,
        );
        let (first, wall) = crate::fighting::met_first(
            order,
            state,
            &mut caches,
            (&talkers, &cell_scripts, &collision, &rigs),
            (from, dir, length),
            bolt.shooter,
            now,
        );
        let at = |d: f32| [0, 1, 2].map(|k| from[k] + dir[k] * d);
        if let Some((d, victim, part)) = first {
            bolt.impact(at(d));
            strike(
                order,
                &scripts,
                state,
                &mut caches,
                (&mut hits, &mut messages),
                &bolt,
                (from, dir, d, out + d),
                (victim, part),
            );
            continue;
        }
        if let Some(wall_at) = wall {
            bolt.impact(at(wall_at.0));
            let at = wall_at;
            hits.shot_on_world(
                &collision.0,
                (from, dir),
                at,
                bolt.shooter,
                bolt.pellet.form_id,
            );
            println!(
                "{now:.1} s: {}'s bolt struck the world {:.0} units out.",
                bolt.shooter,
                out + at.0
            );
            continue;
        }
        bolt.missile.advance(stretch, dt);
        if !bolt.missile.spent(age_max) {
            flying.push(bolt);
        }
    }
    bolts.0 = flying;
}

/// A bolt strikes someone or a scripted object `d` along its frame's way
/// from `from`, `out` units from where it was fired: the hit path, a
/// critical's message for the player's, the hit report, a line.
#[allow(clippy::too_many_arguments)]
fn strike(
    order: &esm::LoadOrder,
    scripts: &Scripts,
    state: &mut world::scripting::GameState,
    caches: &mut crate::combat::PlayerAttack,
    (hits, messages): (
        &mut crate::hiteffects::HitReports,
        &mut crate::hud::HudMessages,
    ),
    bolt: &Bolt,
    (from, dir, d, out): ([f32; 3], [f32; 3], f32, f32),
    (victim, part): (FormId, Option<u8>),
) {
    let me = bolt.shooter;
    let alive = !state.dead.contains(&victim);
    let sneak_attack = me == PLAYER_REF
        && state.player_sneaking
        && world::scripting::Facts {
            order,
            state,
            speaker: None,
        }
        .detection(victim, PLAYER_REF)
        .is_none_or(|v| v < 1);
    let Some(hit) =
        Runner::new(order, &scripts.0, state).hit_at(me, victim, Some(&bolt.pellet), part)
    else {
        println!("{me}'s bolt hit {victim} at {out:.0} units.");
        return;
    };
    if me == PLAYER_REF && hit.critical && alive {
        crate::combat::critical_message(order, state, messages, victim, sneak_attack);
    }
    hits.0.push(crate::hiteffects::HitReport {
        attacker: me,
        target: Some(victim),
        weapon: Some(bolt.pellet.form_id),
        point: [0, 1, 2].map(|k| from[k] + dir[k] * d),
        havok: None,
        normal: None,
        triangle: None,
        direction: dir,
        on_body: true,
        part,
        damage: hit.dealt,
        killed: state.dead.contains(&victim),
    });
    println!(
        "{me}'s bolt: {}.",
        crate::combat::tell_hit(order, state, caches, victim, out, &hit)
    );
}

/// A corpse whose critical stage culls its 3D (the goo's and the
/// disintegration's end, `008a1a70`, `world::more_functions::body_gone`)
/// stops being drawn (the pile `AttachAshPile` leaves is drawn as any
/// reference a script makes).
pub fn hide_culled_bodies(
    dialogue: Res<DialogueState>,
    mut walkers: Query<(&Walker, &mut Visibility)>,
) {
    let state = &dialogue.0;
    if state.more.critical_stage.is_empty() {
        return;
    }
    for (walker, mut visibility) in &mut walkers {
        if *visibility != Visibility::Hidden
            && world::more_functions::body_gone(state, walker.reference)
        {
            *visibility = Visibility::Hidden;
            println!("{} is gone (critical stage).", walker.reference);
        }
    }
}

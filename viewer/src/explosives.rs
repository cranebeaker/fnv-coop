//! Grenades, dynamite and their explosions (`world::explosions`, read from
//! the game's code; evidence in `docs/EXPLOSIVES.md`). The player's attack
//! (`combat`) and people's (`fighting`) throw a weapon of animation type
//! 10–13 here instead of shooting it: the throw uses up one of the weapon,
//! and its projectile flies under gravity through the cell's collision
//! until its fuse runs out (dynamite: 2.5 s) or, for impact grenades, it
//! strikes something. Its explosion hurts everyone alive within its radius
//! and line of sight by its damage × the falloff, through the hit path
//! (`OnHit`, armour, health, `OnDeath`, fighting back), and plays its two
//! sounds; the hits are reported to `hiteffects`.
//!
//! People throw at the point their projectile's arc lands on the target
//! (its feet, for splash damage), low first, else high at
//! `fGrenadeHighArcSpeedPercentage` of the speed when the low arc meets
//! something before `fGrenadeThrowHitFractionThreshold` of the way; with
//! neither clear they don't throw.
//!
//! Measured here (guesses where the game's way isn't traced): the player
//! throws from the eye along the view, people from 60 units above their
//! feet (the hand node isn't posed); the throw leaves at once (not at the
//! throw animation's release); bodies' line-of-sight offsets use the
//! people's controller radius; targets are aimed at as 128 units tall. Not
//! drawn: the projectile's and the explosion's models, its light, image
//! space, decals, camera shake; not done: the limbs the blast reaches, the
//! force it pushes bodies and objects with, knockdowns, mines' proximity,
//! hits on objects (destructibles).

use std::sync::Mutex;

use bevy::prelude::*;
use esm::FormId;
use world::combat::Weapon;
use world::dialogue::PLAYER_REF;
use world::explosions::{
    self, ExplosionRecord, Flight, FlightEvent, FlightSettings, ProjectileRecord,
};
use world::scripting::Runner;

use crate::dialogue::{DialogueState, Talkers};
use crate::scripts::Scripts;
use crate::sounds::SoundRequests;
use crate::walk::{CellCollision, Player};
use crate::GameFiles;

/// How high above the feet people throw from (see the module notes).
pub(crate) const THROW_HEIGHT: f32 = 60.0;

/// How tall a target is taken to be for the AI's aim (see the notes).
const TARGET_HEIGHT: f32 = 128.0;

/// Where a throw goes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Aim {
    /// Along a direction (unit): the player's view.
    Along([f32; 3]),
    /// To land at a target standing at this point (the AI).
    At([f32; 3]),
}

/// A throw asked for by an attack.
#[derive(Debug, Clone)]
pub(crate) struct Launch {
    pub thrower: FormId,
    pub weapon: Weapon,
    pub origin: [f32; 3],
    pub aim: Aim,
}

/// Throws asked for this frame. The attacks are in systems that can't take
/// more parameters (`fighting` runs inside `ai`'s), so they queue here.
static QUEUE: Mutex<Vec<Launch>> = Mutex::new(Vec::new());

/// Projectiles that went off where they struck (`bolts`: a missile with
/// an explosion and no alt. trigger explodes at its impact, `009c3190`):
/// who fired, with what, the projectile and where.
type Detonation = (Cause, Option<Weapon>, ProjectileRecord, [f32; 3]);

/// An explosion's actor cause (none for a placed mine) and whom its hit
/// reports name as the attacker.
type Cause = (Option<FormId>, FormId);
static DETONATIONS: Mutex<Vec<Detonation>> = Mutex::new(Vec::new());

/// Queues an explosion at an impact for [`fly_thrown`].
pub(crate) fn detonate(by: FormId, weapon: Weapon, projectile: ProjectileRecord, at: [f32; 3]) {
    if let Ok(mut q) = DETONATIONS.lock() {
        q.push(((Some(by), by), Some(weapon), projectile, at));
    }
}

/// Queues a throw for [`fly_thrown`].
pub(crate) fn throw(launch: Launch) {
    if let Ok(mut q) = QUEUE.lock() {
        q.push(launch);
    }
}

/// A projectile in flight, and who threw it with what; a mine laid keeps
/// what its proximity test remembers (`world::mines::Mine`).
struct InFlight {
    thrower: FormId,
    weapon: Weapon,
    flight: Flight,
    settings: FlightSettings,
    mine: Option<world::mines::Mine>,
}

/// What's in flight.
#[derive(Resource, Default)]
pub struct Thrown(Vec<InFlight>);

/// A placed mine with its fuse once set off and the time to its next
/// blink.
type LiveMine = (world::mines::Placed, Option<f32>, f32);

/// The placed mines (`PGRE`) of the place the player is in
/// (`world::mines::placed_in`).
#[derive(Resource, Default)]
pub struct PlacedMines {
    space: Option<FormId>,
    mines: Vec<LiveMine>,
}

pub struct ExplosivesPlugin;

impl Plugin for ExplosivesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Thrown>()
            .init_resource::<PlacedMines>()
            .add_systems(
                Update,
                // Thrown things are projectiles, moved in the actors'
                // movement pass (`0096db30`: `009bec10` on the list's
                // projectiles, `009ae580` on its explosions), on the AI
                // threads (`crate::frame_order::AiSet`), under its gate: they
                // hold still in menu mode. After the player's attack (stage 2)
                // by the stages' order.
                fly_thrown
                    // Kept: after people's moves (same pass, ahead of it),
                    // before the hits' effects.
                    .after(crate::ai::move_actors)
                    .before(crate::hiteffects::play_hits)
                    .in_set(crate::frame_order::AiSet::Call(
                        crate::frame_order::ACTORS_MOVEMENT,
                    )),
            );
    }
}

/// A surface's unit normal, facing against `dir`.
fn facing_normal(collider: &physics::Collider, tri: u32, dir: [f32; 3]) -> [f32; 3] {
    let [a, b, c] = collider.triangle(tri);
    let (u, v) = (
        [b[0] - a[0], b[1] - a[1], b[2] - a[2]],
        [c[0] - a[0], c[1] - a[1], c[2] - a[2]],
    );
    let n = [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ];
    let l = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt().max(1e-9);
    let n = n.map(|x| x / l);
    if n[0] * dir[0] + n[1] * dir[1] + n[2] * dir[2] > 0.0 {
        n.map(|x| -x)
    } else {
        n
    }
}

/// The explosion's closest surface and its normal (`009ae6e0`,
/// `world::explosions::closest_surface`): the collider's triangles within
/// the blast's sphere (its phantom's contacts), each at its closest point
/// to the centre with the normal from that point toward the centre (the
/// triangle's own normal facing up when the centre lies on it: unresolved).
/// The contacts are tried nearest first: unresolved, the order Havok's
/// collector gives isn't traced. People aren't triangles here, so every
/// contact and every body met counts as a non-actor's.
fn closest_surface(
    collider: &physics::Collider,
    center: [f32; 3],
    radius: f32,
    skip: &dyn Fn(u32) -> bool,
) -> Option<([f32; 3], [f32; 3])> {
    let lo = center.map(|c| c - radius);
    let hi = center.map(|c| c + radius);
    let mut contacts: Vec<(f32, explosions::Contact)> = collider
        .near(lo, hi)
        .into_iter()
        .filter(|&tri| !skip(tri))
        .filter_map(|tri| {
            let point = closest_on_triangle(center, collider.triangle(tri));
            let to = [0, 1, 2].map(|k| center[k] - point[k]);
            let d = (to[0] * to[0] + to[1] * to[1] + to[2] * to[2]).sqrt();
            if d > radius {
                return None;
            }
            let normal =
                normalize(to).unwrap_or_else(|| facing_normal(collider, tri, [0.0, 0.0, -1.0]));
            Some((
                d,
                explosions::Contact {
                    body: collider.owner(tri),
                    point,
                    normal,
                },
            ))
        })
        .collect();
    contacts.sort_by(|a, b| a.0.total_cmp(&b.0));
    let contacts: Vec<explosions::Contact> = contacts.into_iter().map(|(_, c)| c).collect();
    explosions::closest_surface(center, &contacts, &mut |from, ray| {
        let len = (ray[0] * ray[0] + ray[1] * ray[1] + ray[2] * ray[2]).sqrt();
        let dir = normalize(ray)?;
        let mut along = 0.0;
        loop {
            let start = [0, 1, 2].map(|k| from[k] + dir[k] * along);
            let (d, tri) = collider.raycast(start, dir, len - along)?;
            if !skip(tri) {
                return Some(collider.owner(tri));
            }
            along += d + 0.5;
            if along >= len {
                return None;
            }
        }
    })
}

/// The point of a triangle nearest `p` (Ericson, Real-Time Collision
/// Detection, 5.1.5).
fn closest_on_triangle(p: [f32; 3], [a, b, c]: [[f32; 3]; 3]) -> [f32; 3] {
    let sub = |x: [f32; 3], y: [f32; 3]| [x[0] - y[0], x[1] - y[1], x[2] - y[2]];
    let dot = |x: [f32; 3], y: [f32; 3]| x[0] * y[0] + x[1] * y[1] + x[2] * y[2];
    let at = |o: [f32; 3], d: [f32; 3], t: f32| [o[0] + d[0] * t, o[1] + d[1] * t, o[2] + d[2] * t];
    let (ab, ac, ap) = (sub(b, a), sub(c, a), sub(p, a));
    let (d1, d2) = (dot(ab, ap), dot(ac, ap));
    if d1 <= 0.0 && d2 <= 0.0 {
        return a;
    }
    let bp = sub(p, b);
    let (d3, d4) = (dot(ab, bp), dot(ac, bp));
    if d3 >= 0.0 && d4 <= d3 {
        return b;
    }
    let vc = d1 * d4 - d3 * d2;
    if vc <= 0.0 && d1 >= 0.0 && d3 <= 0.0 {
        return at(a, ab, d1 / (d1 - d3));
    }
    let cp = sub(p, c);
    let (d5, d6) = (dot(ab, cp), dot(ac, cp));
    if d6 >= 0.0 && d5 <= d6 {
        return c;
    }
    let vb = d5 * d2 - d1 * d6;
    if vb <= 0.0 && d2 >= 0.0 && d6 <= 0.0 {
        return at(a, ac, d2 / (d2 - d6));
    }
    let va = d3 * d6 - d5 * d4;
    if va <= 0.0 && (d4 - d3) >= 0.0 && (d5 - d6) >= 0.0 {
        return at(b, sub(c, b), (d4 - d3) / ((d4 - d3) + (d5 - d6)));
    }
    let denom = 1.0 / (va + vb + vc);
    let (v, w) = (vb * denom, vc * denom);
    [0, 1, 2].map(|k| a[k] + ab[k] * v + ac[k] * w)
}

fn normalize(v: [f32; 3]) -> Option<[f32; 3]> {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    (l > 1e-6).then(|| v.map(|x| x / l))
}

/// The direction and speed for an AI's throw at `target` (see the module
/// notes): the low arc, else the high one, if either is clear.
fn ai_throw(
    order: &esm::LoadOrder,
    collider: &physics::Collider,
    projectile: &ProjectileRecord,
    explosion: Option<&ExplosionRecord>,
    (origin, target): ([f32; 3], [f32; 3]),
    speed: f32,
) -> Option<([f32; 3], f32)> {
    let s = |n: &str, d: f32| world::scripting::game_setting(order, n).unwrap_or(d);
    let g = world::combat_ai::WORLD_GRAVITY * projectile.fall_gravity();
    let splash = (
        s("fCombatSplashDamageMaxSpeed", 3000.0),
        s("fCombatSplashDamageMinRadius", 50.0),
        s("fCombatSplashDamageMinDamage", 20.0),
    );
    let blast = explosion.map(|e| (e.radius_units(s("fBSUnitsPerFoot", 22.0)), e.damage));
    let segments = s("iBallisticProjectilePathPickSegments", 4.0) as u32;
    let threshold = s("fGrenadeThrowHitFractionThreshold", 0.8);
    let mut cast =
        |from: [f32; 3], dir: [f32; 3], len: f32| collider.raycast(from, dir, len).map(|(d, _)| d);
    for high in [false, true] {
        let mode = explosions::aim_mode(projectile, blast, high, splash);
        let point = [
            target[0],
            target[1],
            target[2] + explosions::aim_height(mode, TARGET_HEIGHT),
        ];
        let v = if high {
            speed * s(explosions::HIGH_ARC_SETTING, 0.67)
        } else {
            speed
        };
        let aim = explosions::aim_point(origin, point, v, g, high);
        let Some(dir) = normalize([aim[0] - origin[0], aim[1] - origin[1], aim[2] - origin[2]])
        else {
            continue;
        };
        if explosions::arc_clear(origin, dir, (v, g), point, (segments, threshold), &mut cast) {
            return Some((dir, v));
        }
    }
    None
}

/// One of the weapon used up by a throw; the last one leaves the hand.
fn use_up(order: &esm::LoadOrder, state: &mut world::scripting::GameState, who: FormId, w: FormId) {
    state.stock(order, who);
    let n = state.items.entry((who, w)).or_insert(0);
    *n = (*n - 1).max(0);
    if *n == 0 {
        state.unequip(who, w);
    }
}

/// Launches queued throws, flies what's in flight, and sets off what goes
/// off.
#[allow(clippy::too_many_arguments)]
pub fn fly_thrown(
    time: Res<Time>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    mut state: ResMut<DialogueState>,
    (collision, talkers, player): (Res<CellCollision>, Res<Talkers>, Res<Player>),
    mut sounds: ResMut<SoundRequests>,
    mut hits: ResMut<crate::hiteffects::HitReports>,
    (mut thrown, mut placed): (ResMut<Thrown>, ResMut<PlacedMines>),
) {
    let order = &game.0.order;
    let state = &mut state.0;
    let collider = &collision.0;
    let launches: Vec<Launch> = QUEUE
        .lock()
        .map(|mut q| std::mem::take(&mut *q))
        .unwrap_or_default();
    for l in launches {
        let Some(projectile) = l
            .weapon
            .projectile
            .and_then(|p| ProjectileRecord::load(order, p))
        else {
            continue;
        };
        let explosion = projectile
            .explosion
            .and_then(|e| ExplosionRecord::load(order, e));
        let speed = explosions::launch_speed(order, state, l.thrower, &l.weapon, &projectile, 1.0);
        let (dir, speed) = match l.aim {
            Aim::Along(d) => (d, speed),
            Aim::At(target) => {
                match ai_throw(
                    order,
                    collider,
                    &projectile,
                    explosion.as_ref(),
                    (l.origin, target),
                    speed,
                ) {
                    Some(t) => t,
                    None => {
                        println!(
                            "{:.1} s: {} holds the {}: no clear arc.",
                            time.elapsed_secs(),
                            l.thrower,
                            l.weapon.name
                        );
                        continue;
                    }
                }
            }
        };
        use_up(order, state, l.thrower, l.weapon.form_id);
        println!(
            "{:.1} s: {} throws the {} at {speed:.0} units a second ({:.1} s fuse).",
            time.elapsed_secs(),
            l.thrower,
            l.weapon.name,
            projectile.timer
        );
        let settings = FlightSettings::read(order, &projectile);
        let mine = projectile.is_mine().then(|| world::mines::Mine {
            projectile: projectile.clone(),
            shooter: Some(l.thrower),
            owner: None,
            position: l.origin,
            exterior: false,
            spares_player: false,
            disarmed: false,
        });
        thrown.0.push(InFlight {
            thrower: l.thrower,
            weapon: l.weapon,
            flight: Flight::launch(projectile, l.origin, dir, speed),
            settings,
            mine,
        });
    }
    let mut going_off: Vec<Detonation> = DETONATIONS
        .lock()
        .map(|mut q| std::mem::take(&mut *q))
        .unwrap_or_default();
    let dt = time.delta_secs();
    let feet = player.character.feet;
    // Mines wait for the player to be placed (the first frames start
    // them elsewhere).
    if player.ready {
        tend_mines(
            order,
            state,
            (&talkers, feet, time.elapsed_secs(), dt),
            (&mut placed, &mut thrown),
            &mut sounds,
            &mut going_off,
        );
    }
    if thrown.0.is_empty() && going_off.is_empty() {
        return;
    }
    let mut cast = |from: [f32; 3], dir: [f32; 3], len: f32| {
        collider
            .raycast(from, dir, len)
            .map(|(d, tri)| (d, facing_normal(collider, tri, dir)))
    };
    thrown
        .0
        .retain_mut(|f| match f.flight.step(dt, &f.settings, &mut cast) {
            FlightEvent::Flying => true,
            FlightEvent::Expired => false,
            FlightEvent::Explode { at } => {
                going_off.push((
                    (Some(f.thrower), f.thrower),
                    Some(f.weapon.clone()),
                    f.flight.projectile.clone(),
                    at,
                ));
                false
            }
        });
    for (cause, weapon, projectile, at) in going_off {
        let Some(e) = projectile
            .explosion
            .and_then(|e| ExplosionRecord::load(order, e))
        else {
            continue;
        };
        explode(
            order,
            &scripts.0,
            state,
            collider,
            &talkers,
            feet,
            (cause, weapon.as_ref(), &e, at),
            &mut sounds,
            &mut hits,
        );
    }
}

/// The mines (`world::mines`): the place's placed ones (read again when
/// the player's place changes) and the laid ones in flight or lying go
/// off for whoever comes near; a set-off fuse counts down, blinking
/// faster, and plays the countdown sound; at 0 it's queued to explode.
/// Placed mines disarmed or taken (`scripts`) stop.
fn tend_mines(
    order: &esm::LoadOrder,
    state: &mut world::scripting::GameState,
    (talkers, feet, now, dt): (&Talkers, [f32; 3], f32, f32),
    (placed, thrown): (&mut PlacedMines, &mut Thrown),
    sounds: &mut SoundRequests,
    going_off: &mut Vec<Detonation>,
) {
    let space = state.place(order, PLAYER_REF).map(|p| p.0);
    if placed.space != space {
        placed.space = space;
        placed.mines = space
            .map(|s| world::mines::placed_in(order, state, s))
            .unwrap_or_default()
            .into_iter()
            .map(|p| (p, None, 0.0))
            .collect();
        if !placed.mines.is_empty() {
            println!("{} mines lie here.", placed.mines.len());
        }
    }
    let gone = &state.more.mines.gone;
    placed.mines.retain(|(p, ..)| !gone.contains(&p.reference));
    let thrown_mines = thrown.0.iter().any(|f| f.mine.is_some());
    if placed.mines.is_empty() && !thrown_mines {
        return;
    }
    let exterior = space
        .and_then(|s| order.get(s))
        .is_some_and(|r| r.entry.header.kind == esm::FourCC::new(b"WRLD"));
    let people: Vec<(FormId, [f32; 3])> = talkers
        .0
        .iter()
        .filter(|t| !state.dead.contains(&t.reference))
        .map(|t| (t.reference, t.position))
        .collect();
    let player = (!state.dead.contains(&PLAYER_REF)).then_some(feet);
    let s = world::mines::MineSettings::read(order);
    for (p, fuse, blink) in &mut placed.mines {
        if state.more.mines.disarmed.contains(&p.reference) && !p.mine.disarmed {
            p.mine.disarmed = true;
            *fuse = None;
        }
        let Some(f) = fuse.as_mut() else {
            let roll = state.roll();
            let set =
                world::mines::check_proximity(order, state, &s, &mut p.mine, &people, player, roll);
            if let Some(f) = set {
                println!("{now:.1} s: mine {} is set off ({f:.2} s).", p.reference);
                state.more.mines.fuse_running.insert(p.reference);
                sounds.0.extend(p.mine.projectile.countdown_sound);
                *fuse = Some(f);
                *blink = world::mines::blink_interval(&s, f);
            }
            continue;
        };
        *f -= dt;
        *blink -= dt;
        if *blink <= 0.0 {
            // Each blink starts the countdown sound again (`009c3190`).
            *blink = world::mines::blink_interval(&s, f.max(0.0));
            sounds.0.extend(p.mine.projectile.countdown_sound);
        }
        if *f <= 0.0 {
            world::mines::gone(state, p.reference);
            going_off.push((
                (None, p.reference),
                None,
                p.mine.projectile.clone(),
                p.mine.position,
            ));
        }
    }
    for f in &mut thrown.0 {
        let Some(m) = f.mine.as_mut().filter(|_| f.flight.fuse.is_none()) else {
            continue;
        };
        m.position = f.flight.position;
        m.exterior = exterior;
        let roll = state.roll();
        if let Some(fuse) =
            world::mines::check_proximity(order, state, &s, m, &people, player, roll)
        {
            println!("{now:.1} s: {}'s mine is set off ({fuse:.2} s).", f.thrower);
            sounds.0.extend(m.projectile.countdown_sound);
            f.flight.set_off(fuse);
        }
    }
}

/// An explosion at `at`: its sounds, and its hits on everyone it reaches.
#[allow(clippy::too_many_arguments)]
fn explode(
    order: &esm::LoadOrder,
    scripts: &world::scripting::ScriptCache,
    state: &mut world::scripting::GameState,
    collider: &physics::Collider,
    talkers: &Talkers,
    player_feet: [f32; 3],
    (cause, weapon, e, at): (Cause, Option<&Weapon>, &ExplosionRecord, [f32; 3]),
    sounds: &mut SoundRequests,
    hits: &mut crate::hiteffects::HitReports,
) {
    sounds.0.extend(e.sound);
    sounds.0.extend(e.sound2);
    // The actor that caused it (none for a placed mine) and what the hit
    // reports name as its attacker.
    let (thrower, reporter) = cause;
    let radius = explosions::blast_radius(order, state, thrower, weapon.map(|w| w.form_id), e);
    let damage = explosions::base_damage(order, state, thrower, weapon, e);
    // It pushes the clutter in its sphere (`clutter`).
    crate::clutter::blast(e, at, radius);
    println!(
        "{} goes off at ({:.0}, {:.0}, {:.0}): {damage:.1} damage, {radius:.0} units.",
        e.form_id, at[0], at[1], at[2]
    );
    let mut candidates: Vec<(FormId, [f32; 3])> = talkers
        .0
        .iter()
        .filter(|t| !state.dead.contains(&t.reference))
        .filter(|t| !world::more_functions::is_ghost(state, t.reference))
        .map(|t| (t.reference, t.position))
        .collect();
    if !state.dead.contains(&PLAYER_REF) {
        candidates.push((PLAYER_REF, player_feet));
    }
    // A placed mine's own collision is left out of the casts: unresolved,
    // the exploding projectile's removal isn't traced.
    let own = |tri: u32| collider.owner(tri) == reporter.0 && thrower.is_none();
    let mut cast = |from: [f32; 3], dir: [f32; 3], len: f32| {
        let mut along = 0.0;
        loop {
            let start = [0, 1, 2].map(|k| from[k] + dir[k] * along);
            let (d, tri) = collider.raycast(start, dir, len - along)?;
            if !own(tri) {
                return Some(along + d);
            }
            along += d + 0.5;
            if along >= len {
                return None;
            }
        }
    };
    // The pick's start (`009b1810`): off the surface the blast touches
    // (`009ae6e0`, `world::explosions::closest_surface`).
    let normal = closest_surface(collider, at, radius, &own).map_or([0.0; 3], |(_, n)| n);
    let pick = explosions::LosPick {
        normal,
        buffer_distance: world::scripting::game_setting(order, "fExplosionLOSBufferDistance")
            .unwrap_or(24.0),
        buffer: world::scripting::game_setting(order, "fExplosionLOSBuffer").unwrap_or(6.0),
        // Projectiles' and mines' explosions have no owner (`009c3190`).
        owner_passes: false,
    };
    for t in explosions::blast_targets(at, radius, &candidates) {
        let Some(&(_, position)) = candidates.iter().find(|c| c.0 == t.reference) else {
            continue;
        };
        let sees = e.flags & explosions::expl_flags::IGNORE_LOS != 0
            || explosions::los_clear(
                at,
                position,
                Some(world::combat_ai::PERSON_RADIUS),
                &pick,
                &mut cast,
            );
        if !sees {
            println!("  {} is shielded from it.", t.reference);
            continue;
        }
        let Some(hit) = Runner::new(order, scripts, state).explosion_hit(
            thrower,
            t.reference,
            weapon,
            damage * t.share,
        ) else {
            continue;
        };
        // Its object effect (`EMP`, fire), then whether it knocks them
        // down (`009b00a0`; the fall itself is the physics').
        for said in explosions::cast_enchantment(order, state, e, thrower, t.reference) {
            println!("  {}: {said}", t.reference);
        }
        let roll = state.roll();
        let down = explosions::knocks_down(order, state, e, t.reference, damage * t.share, roll);
        let killed = state.dead.contains(&t.reference);
        let left = world::combat::health(order, state, t.reference)
            .unwrap_or(0.0)
            .max(0.0);
        println!(
            "  {} at {:.0} units takes {:.1} ({left:.1} left){}{}.",
            t.reference,
            t.distance,
            hit.dealt,
            if killed { ", killed" } else { "" },
            if down { ", knocked down" } else { "" }
        );
        hits.0.push(crate::hiteffects::HitReport {
            attacker: reporter,
            target: Some(t.reference),
            // The explosion's hit carries its weapon source (`009b5770`:
            // hit `+0x30` = `Explosion::pWeaponSource`, `+0xe0`,
            // `009b0900`), whose impact set the hit sounds come from
            // (`0088e1e0`) as for any hit.
            weapon: weapon.map(|w| w.form_id),
            point: position,
            havok: None,
            normal: None,
            triangle: None,
            direction: [0.0; 3],
            on_body: false,
            part: None,
            damage: hit.dealt,
            killed,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_grenade_on_the_floor_finds_the_floor_below_it() {
        let mut c = physics::Collider::new();
        c.add(
            &[
                [-100.0, -100.0, 0.0],
                [100.0, -100.0, 0.0],
                [0.0, 100.0, 0.0],
            ],
            &[[0, 1, 2]],
        );
        let (point, normal) = closest_surface(&c, [0.0, 0.0, 3.0], 50.0, &|_| false).unwrap();
        assert_eq!(point, [0.0, 0.0, 0.0]);
        assert_eq!(normal, [0.0, 0.0, 1.0]);
        // Out of the sphere: none.
        assert!(closest_surface(&c, [0.0, 0.0, 80.0], 50.0, &|_| false).is_none());
    }

    #[test]
    fn normals_face_the_ray() {
        let mut c = physics::Collider::new();
        c.add(
            &[
                [-100.0, -100.0, 0.0],
                [100.0, -100.0, 0.0],
                [0.0, 100.0, 0.0],
            ],
            &[[0, 1, 2]],
        );
        let (_, tri) = c
            .raycast([0.0, 0.0, 50.0], [0.0, 0.0, -1.0], 100.0)
            .unwrap();
        assert_eq!(facing_normal(&c, tri, [0.0, 0.0, -1.0]), [0.0, 0.0, 1.0]);
        assert_eq!(facing_normal(&c, tri, [0.0, 0.0, 1.0]), [0.0, 0.0, -1.0]);
    }

    #[test]
    fn queued_throws_are_taken_once() {
        let w = Weapon {
            form_id: FormId(1),
            name: "Dynamite".into(),
            damage: 1.0,
            clip: 0,
            health: 0,
            animation: 10,
            ammo: Vec::new(),
            ammo_use: 1,
            min_spread: 0.0,
            spread: 0.0,
            projectile: None,
            projectiles: 1,
            min_range: 0.0,
            max_range: 0.0,
            shots_per_second: 1.0,
            reload_time: 1.0,
            skill: 33,
            crit_damage: 0.0,
            crit_mult: 1.0,
            sound: None,
            attack_animation: 0,
            reload_animation: 0,
            kill_impulse: 0.0,
            impulse_distance: 0.0,
            reach: 0.0,
            limb_damage_mult: 1.0,
            flags1: 0,
            flags2: 0,
            fire_rate: 0.0,
            attack_mult: 1.0,
            aim_arc: 0.0,
            semi_auto_delay: (0.0, 0.0),
            speed: 1.0,
            cone_mult: 1.0,
            crit_effect: None,
            crit_on_death: false,
            resist: None,
        };
        throw(Launch {
            thrower: PLAYER_REF,
            weapon: w,
            origin: [0.0; 3],
            aim: Aim::Along([0.0, 1.0, 0.0]),
        });
        let taken: Vec<Launch> = std::mem::take(&mut *QUEUE.lock().unwrap());
        assert_eq!(taken.len(), 1);
        assert!(QUEUE.lock().unwrap().is_empty());
    }
}

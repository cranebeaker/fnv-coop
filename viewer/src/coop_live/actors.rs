//! Shared actors: every peer reports the people and creatures of the loaded
//! cells (the host replicates them, a guest tells the host), owners publish
//! their animation ids, and everyone else shows the owner's copy. An actor
//! only another peer has loaded gets an entity here too ([`adopt_remote`]).
//! Death is not handled here: a dead actor is released to the ordinary AI
//! path (`ai::move_actors` lets it fall) as soon as the state says it died.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use coop_session::ActorView;
use esm::FormId;
use world::coop::apply::Ctx;

use super::frame::{ANIM_FIGHTING, ANIM_MOVING, ANIM_RUNNING, ANIM_SNEAKING};
use super::puppets::placed;
use super::{CoopPuppet, LiveNow};
use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::coop::CoopLink;
use crate::dialogue::DialogueState;
use crate::scripts::Scripts;
use crate::{GameFiles, Spawner};

/// Units a second above which a copy walks, and runs.
const WALK_SPEED: f32 = 5.0;
const RUN_SPEED: f32 = 220.0;

/// Starts replicating the people and creatures on screen, once each (the
/// host registers them; a guest reports them to the host).
pub fn track(
    time: Res<Time>,
    mut link: NonSendMut<CoopLink>,
    state: Res<DialogueState>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    walkers: Query<&Walker, Without<CoopPuppet>>,
    mut known: Local<HashSet<FormId>>,
    mut last: Local<f32>,
) {
    let t = time.elapsed_secs();
    if t - *last < 1.0 {
        return;
    }
    *last = t;
    let new: Vec<FormId> = walkers
        .iter()
        .map(|w| w.reference)
        .filter(|r| !known.contains(r))
        .collect();
    if new.is_empty() {
        return;
    }
    let cx = Ctx {
        order: &game.0.order,
        scripts: &scripts.0,
    };
    // false: the layer cannot take them yet (a guest still joining)
    if link.0.live_track(cx, &state.0, &new) {
        known.extend(new);
    }
}

/// Owners publish what their actors are doing (the animation id).
pub fn publish(
    now: Res<LiveNow>,
    mut state: ResMut<DialogueState>,
    rigs: Query<(&Walker, &ActorRig), Without<CoopPuppet>>,
) {
    let Some(view) = &now.view else { return };
    if view.owned_actors.is_empty() {
        return;
    }
    let owned: HashSet<FormId> = view.owned_actors.iter().copied().collect();
    for (w, rig) in &rigs {
        if !owned.contains(&w.reference) {
            continue;
        }
        let mut anim = 0u16;
        if rig.walking {
            anim |= ANIM_MOVING;
        }
        if rig.running {
            anim |= ANIM_RUNNING;
        }
        if rig.sneaking {
            anim |= ANIM_SNEAKING;
        }
        if rig.fighting {
            anim |= ANIM_FIGHTING;
        }
        state.0.coop.live.anim.insert(w.reference, anim);
    }
}

/// Before the animations: the local copy of an actor another peer owns
/// stands where the owner says, and the AI leaves it alone.
#[allow(clippy::too_many_arguments)]
pub fn apply_remote(
    time: Res<Time>,
    now: Res<LiveNow>,
    state: Res<DialogueState>,
    mut commands: Commands,
    mut seen: Local<HashMap<FormId, ([f32; 3], [f32; 3])>>,
    mut people: Query<(
        Entity,
        &mut Walker,
        &mut ActorRig,
        &mut Transform,
        Option<&CoopPuppet>,
    )>,
) {
    let Some(view) = &now.view else { return };
    let remote: HashMap<FormId, &ActorView> =
        view.remote_actors.iter().map(|a| (a.id, a)).collect();
    let dt = time.delta_secs().max(1e-3);
    for (e, mut walker, mut rig, mut transform, mark) in &mut people {
        if mark.is_some_and(|m| m.0.is_some()) {
            continue;
        }
        let me = walker.reference;
        match remote.get(&me) {
            // an owner with no place for it yet publishes the origin: leave it to the AI
            Some(a) if !a.dead && !state.0.dead.contains(&me) && a.pos != [0.0; 3] => {
                if mark.is_none() {
                    commands.entity(e).insert(CoopPuppet(None));
                }
                let entry = seen.entry(me).or_insert((a.pos, [0.0; 3]));
                let raw = [0, 1, 2].map(|i| (a.pos[i] - entry.0[i]) / dt);
                let v = [0, 1, 2].map(|i| entry.1[i] * 0.6 + raw[i] * 0.4);
                *entry = (a.pos, v);
                let speed = v[0].hypot(v[1]);
                walker.position = a.pos;
                walker.heading = a.heading;
                walker.velocity = v;
                rig.walking = speed > WALK_SPEED || a.anim & ANIM_MOVING != 0;
                rig.running = speed > RUN_SPEED || a.anim & ANIM_RUNNING != 0;
                rig.sneaking = a.anim & ANIM_SNEAKING != 0;
                rig.speed = speed;
                rig.fighting = a.anim & ANIM_FIGHTING != 0;
                *transform = placed(&walker);
            }
            _ => {
                if mark.is_some() {
                    commands.entity(e).remove::<CoopPuppet>();
                    seen.remove(&me);
                }
            }
        }
    }
}

/// Outdoors, how far (game units) an actor only another peer has loaded is
/// still brought in (the loaded squares, as `bring_in_people`).
const REACH: f32 = 2.5 * world::land::CELL_SIZE;
/// Seconds before asking again for an actor that did not appear.
const ASK_AGAIN: f32 = 4.0;

/// An actor another peer owns that has no entity here (a creature found in
/// a place only that peer had loaded, or one standing just inside our
/// squares) is brought on screen like any placed person; `apply_remote`
/// then drives it. Made references (`PlaceAtMe`) come through
/// `bring_in_made` instead, from the record the live layer keeps.
#[allow(clippy::too_many_arguments)]
pub fn adopt_remote(
    time: Res<Time>,
    now: Res<LiveNow>,
    state: Res<DialogueState>,
    game: Res<GameFiles>,
    player: Res<crate::walk::Player>,
    walkers: Query<&Walker>,
    mut talkers: ResMut<crate::dialogue::Talkers>,
    mut spawner: Spawner,
    mut asked: Local<HashMap<FormId, f32>>,
    mut last: Local<f32>,
) {
    let t = time.elapsed_secs();
    if !player.ready || t - *last < 1.0 {
        return;
    }
    *last = t;
    let Some(view) = &now.view else { return };
    let gs = &state.0;
    let Some(space) = gs.player_world.or(gs.player_cell) else {
        return;
    };
    let shown: HashSet<FormId> = walkers.iter().map(|w| w.reference).collect();
    let mut new: Vec<FormId> = Vec::new();
    for a in &view.remote_actors {
        if a.dead || a.cell != space.0 || shown.contains(&a.id) || gs.dead.contains(&a.id) {
            continue;
        }
        if gs.more.placed.refs.contains_key(&a.id)
            || asked.get(&a.id).is_some_and(|at| t - at < ASK_AGAIN)
        {
            continue;
        }
        let far = gs.player_world.is_some()
            && gs
                .player_position
                .is_some_and(|me| (a.pos[0] - me[0]).hypot(a.pos[1] - me[1]) > REACH);
        if far {
            continue;
        }
        asked.insert(a.id, t);
        new.push(a.id);
    }
    if new.is_empty() {
        return;
    }
    let scene = game.0.actors_scene(&new);
    for a in &scene.actors {
        println!(
            "Co-op: {} comes into view (shared actor).",
            FormId(a.reference)
        );
    }
    for a in &scene.actors {
        if !talkers.0.iter().any(|x| x.reference.0 == a.reference) {
            talkers.0.push(crate::dialogue::Talker::from_actor(a));
        }
    }
    let lighting = spawner.place_lighting.get();
    spawner.spawn_with(&scene, lighting);
}

/// Counts the spawned people and creatures in the scene, and how many are
/// not hidden, for the state dump (so a run on a real machine can assert
/// that the people are there).
pub fn count(
    time: Res<Time>,
    mut now: ResMut<LiveNow>,
    actors: Query<
        (
            &Visibility,
            Option<&CoopPuppet>,
            Option<&crate::dress::Dressed>,
        ),
        With<Walker>,
    >,
    mut last: Local<f32>,
) {
    let t = time.elapsed_secs();
    if t - *last < 0.5 {
        return;
    }
    *last = t;
    let c = &mut now.counters;
    (
        c.npc_entities,
        c.npc_entities_visible,
        c.creature_entities,
        c.creature_entities_visible,
    ) = (0, 0, 0, 0);
    for (vis, mark, person) in &actors {
        // remote players' puppets are not NPCs
        if mark.is_some_and(|m| m.0.is_some()) {
            continue;
        }
        let shown = u64::from(*vis != Visibility::Hidden);
        if person.is_some() {
            c.npc_entities += 1;
            c.npc_entities_visible += shown;
        } else {
            c.creature_entities += 1;
            c.creature_entities_visible += shown;
        }
    }
}

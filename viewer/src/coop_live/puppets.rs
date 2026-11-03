//! Remote players as actors: one puppet per remote player in view, spawned
//! from the player's own NPC record (`PLAYER_BASE`) at the synthetic form
//! `puppet_of(player)`, dressed in what they wear, driven each frame from
//! the interpolated avatar, with a name tag above it.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use cellview::space;
use coop_session::{PlayerId, RemoteAvatarView};
use esm::FormId;
use world::coop::facts::player_of;
use world::scripting::GameState;

use super::{CoopPuppet, LiveNow};
use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::dialogue::DialogueState;
use crate::{GameFiles, Spawner};

/// The NPC record the player's body is made from.
const PLAYER_BASE: FormId = FormId(0x7);
/// Outdoors, how far (game units) a player in another square is still drawn.
const REACH: f32 = 2.5 * world::land::CELL_SIZE;
/// Seconds to wait for a spawned puppet before asking again.
const RESPAWN_AFTER: f32 = 4.0;
/// Units a second above which a puppet walks, and runs.
const WALK_SPEED: f32 = 5.0;
const RUN_SPEED: f32 = 220.0;
/// Units per metre (for the name tags).
// not read yet: the name tags do not show the distance (the f71db09 feature merge)
#[allow(dead_code)]
const UNITS_PER_METRE: f32 = 70.0;

/// What the glue tracks about the puppets.
#[derive(Resource, Default)]
pub struct Puppets {
    /// Adopted roots.
    entities: HashMap<PlayerId, Entity>,
    /// When a spawn was asked for.
    asked: HashMap<PlayerId, f32>,
    /// Name tag entities.
    tags: HashMap<PlayerId, Entity>,
}

/// A name tag above a remote player.
#[derive(Component)]
pub struct NameTag;

/// Whether the local player can see a remote avatar's place: the same cell,
/// or outdoors near enough (the loaded squares).
fn in_view(gs: &GameState, a: &RemoteAvatarView) -> bool {
    let cell = gs.player_cell.map_or(0, |c| c.0);
    if a.cell != 0 && a.cell == cell {
        return true;
    }
    a.cell != 0
        && gs.player_world.is_some()
        && gs
            .player_position
            .is_some_and(|me| (a.pos[0] - me[0]).hypot(a.pos[1] - me[1]) <= REACH)
}

/// What the puppet wears: the avatar's equipment and its weapon.
fn worn(a: &RemoteAvatarView) -> Vec<FormId> {
    let mut v: Vec<FormId> = a.equipment.iter().map(|(_, f)| *f).collect();
    if a.weapon != 0 {
        v.push(FormId(a.weapon));
    }
    v
}

/// Spawns, adopts and despawns the puppets to match the view.
#[allow(clippy::too_many_arguments)]
pub fn sync(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    now: Res<LiveNow>,
    mut puppets: ResMut<Puppets>,
    mut commands: Commands,
    walkers: Query<(Entity, &Walker, Option<&CoopPuppet>)>,
    mut spawner: Spawner,
) {
    let Some(view) = &now.view else { return };
    let t = time.elapsed_secs();
    let order = &game.0.order;
    let gs = &mut state.0;
    let shown: HashMap<PlayerId, &RemoteAvatarView> = view
        .avatars
        .iter()
        .filter(|a| in_view(gs, a))
        .map(|a| (a.player, a))
        .collect();
    // spawned puppets: mark them so the AI leaves them alone
    for (e, w, mark) in &walkers {
        let Some(p) = player_of(w.reference) else {
            continue;
        };
        if mark.is_none() {
            commands.entity(e).insert(CoopPuppet(Some(p)));
        }
        puppets.entities.insert(p, e);
        puppets.asked.remove(&p);
    }
    // gone with their scene, or gone from view
    let live: HashSet<Entity> = walkers.iter().map(|(e, ..)| e).collect();
    let mut drop: Vec<PlayerId> = Vec::new();
    for (p, e) in &puppets.entities {
        if !live.contains(e) || !shown.contains_key(p) || view.left.contains(p) {
            drop.push(*p);
        }
    }
    for p in drop {
        if let Some(e) = puppets.entities.remove(&p) {
            if live.contains(&e) {
                commands.entity(e).despawn();
            }
        }
        let form = world::coop::facts::puppet_of(p);
        gs.positions.remove(&form);
        gs.equipped.remove(&form);
        gs.more.alerted.remove(&form);
        gs.more.forced_sneak.remove(&form);
    }
    // new ones
    for (p, a) in &shown {
        if puppets.entities.contains_key(p)
            || puppets
                .asked
                .get(p)
                .is_some_and(|at| t - at < RESPAWN_AFTER)
        {
            continue;
        }
        gs.positions.insert(a.puppet, (a.pos, a.heading));
        gs.equipped.insert(a.puppet, worn(a));
        let Some(placement) =
            world::made_placement(order, a.puppet, PLAYER_BASE, a.pos, [0.0, 0.0, a.heading])
        else {
            puppets.asked.insert(*p, t);
            continue;
        };
        println!("Co-op: {} ({}) comes into view.", a.name, a.puppet);
        let scene = game.0.made_scene(vec![placement]);
        let lighting = spawner.place_lighting.get();
        spawner.spawn_with(&scene, lighting);
        puppets.asked.insert(*p, t);
    }
}

/// Per puppet: the last position and the smoothed velocity.
type Motion = HashMap<PlayerId, ([f32; 3], [f32; 3])>;

/// Smoothed velocity of something that moved from `last` to `now` in `dt`.
pub fn velocity(
    motion: &mut Motion,
    key: PlayerId,
    now: [f32; 3],
    dt: f32,
    jumped: bool,
) -> [f32; 3] {
    let dt = dt.max(1e-3);
    let entry = motion.entry(key).or_insert((now, [0.0; 3]));
    let raw = if jumped {
        [0.0; 3]
    } else {
        [
            (now[0] - entry.0[0]) / dt,
            (now[1] - entry.0[1]) / dt,
            (now[2] - entry.0[2]) / dt,
        ]
    };
    let v = [0, 1, 2].map(|i| entry.1[i] * 0.6 + raw[i] * 0.4);
    *entry = (now, v);
    v
}

/// The game-space placement of a walker as a Bevy transform.
pub fn placed(walker: &Walker) -> Transform {
    Transform::from_matrix(Mat4::from_cols_array(&space::matrix(&walker.game_matrix())))
}

/// Before the animations: each puppet takes its avatar's place, pose and
/// what it wears.
#[allow(clippy::too_many_arguments)]
pub fn drive(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    now: Res<LiveNow>,
    mut motion: Local<Motion>,
    mut dressed: Local<HashMap<PlayerId, (u32, u32)>>,
    mut puppets: Query<(&CoopPuppet, &mut Walker, &mut ActorRig, &mut Transform)>,
) {
    let Some(view) = &now.view else { return };
    let order = &game.0.order;
    let gs = &mut state.0;
    let dt = time.delta_secs();
    for (mark, mut walker, mut rig, mut transform) in &mut puppets {
        let Some(p) = mark.0 else { continue };
        let Some(a) = view.avatars.iter().find(|a| a.player == p) else {
            continue;
        };
        let v = velocity(&mut motion, p, a.pos, dt, a.teleported);
        let speed = v[0].hypot(v[1]);
        walker.position = a.pos;
        walker.heading = a.heading;
        walker.velocity = v;
        let me = a.puppet;
        gs.positions.insert(me, (a.pos, a.heading));
        // what it wears and holds, redrawn by `dress` when it changes
        let key = (a.parts_rev, a.weapon);
        if dressed.get(&p) != Some(&key) {
            dressed.insert(p, key);
            gs.equipped.insert(me, worn(a));
        }
        if a.drawn {
            gs.more.alerted.insert(me);
        } else {
            gs.more.alerted.remove(&me);
        }
        if a.sneaking {
            gs.more.forced_sneak.insert(me);
        } else {
            gs.more.forced_sneak.remove(&me);
        }
        // combat events of that player, then the dead lie where they fell
        for (_, ev) in view.actions.iter().filter(|(q, _)| *q == p) {
            super::actions::play(
                &mut rig,
                &walker,
                gs,
                me,
                time.elapsed_secs(),
                ev.kind,
                ev.arg,
            );
        }
        if !a.dead && rig.ragdoll.is_some() {
            rig.ragdoll = None;
        } else if a.dead && rig.ragdoll.is_none() {
            super::actions::play(
                &mut rig,
                &walker,
                gs,
                me,
                time.elapsed_secs(),
                coop_session::ActionKind::Death,
                0,
            );
        }
        let weapon = world::combat::weapon_equipped(order, gs, me);
        rig.character = true;
        rig.still = rig.ragdoll.is_some();
        rig.walking = speed > WALK_SPEED;
        rig.running = speed > RUN_SPEED;
        rig.sneaking = a.sneaking;
        rig.speed = speed;
        rig.fighting = a.drawn;
        rig.held_weapon = Some(weapon.as_ref().map(|w| w.form_id));
        rig.weapon_kind = weapon
            .as_ref()
            .map(|w| world::animation::groups::weapon_kind(w.animation));
        *transform = placed(&walker);
    }
}

/// Name tags above the puppets in view, with the distance in metres.
#[allow(clippy::too_many_arguments)]
pub fn name_tags(
    mut commands: Commands,
    now: Res<LiveNow>,
    state: Res<DialogueState>,
    mut puppets: ResMut<Puppets>,
    cameras: Query<(&Camera, &GlobalTransform), With<crate::FlyCamera>>,
    mut tags: Query<(&mut Node, &mut Text, &mut Visibility, &mut TextColor), With<NameTag>>,
) {
    let Some(view) = &now.view else { return };
    let Ok((camera, camera_at)) = cameras.single() else {
        return;
    };
    let wanted: HashSet<PlayerId> = view
        .avatars
        .iter()
        .filter(|a| puppets.entities.contains_key(&a.player))
        .map(|a| a.player)
        .collect();
    let stale: Vec<PlayerId> = puppets
        .tags
        .keys()
        .filter(|p| !wanted.contains(p))
        .copied()
        .collect();
    for p in stale {
        if let Some(e) = puppets.tags.remove(&p) {
            commands.entity(e).despawn();
        }
    }
    for a in view.avatars.iter().filter(|a| wanted.contains(&a.player)) {
        let (label, color) =
            super::hud_social::tag_text(&now, &state.0, a, super::voice::is_speaking(a.player.0));
        let Some(tag) = puppets.tags.get(&a.player).copied() else {
            let e = commands
                .spawn((
                    Text::new(label),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.95, 0.7)),
                    Node {
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    Visibility::Hidden,
                    NameTag,
                ))
                .id();
            puppets.tags.insert(a.player, e);
            continue;
        };
        let Ok((mut node, mut text, mut vis, mut tint)) = tags.get_mut(tag) else {
            continue;
        };
        if tint.0 != color {
            tint.0 = color;
        }
        let head = Vec3::from(space::point([a.pos[0], a.pos[1], a.pos[2] + 190.0]));
        match camera.world_to_viewport(camera_at, head) {
            Ok(at) => {
                node.left = Val::Px(at.x - 40.0);
                node.top = Val::Px(at.y - 10.0);
                if text.0 != label {
                    text.0 = label;
                }
                *vis = Visibility::Inherited;
            }
            Err(_) => *vis = Visibility::Hidden,
        }
    }
}

//! The player's body in third person: built as people are built
//! (`world::actor::player_look`: the player's record for race, face and
//! hair, the game state for sex, worn clothes and the weapon in hand) on
//! the third-person skeleton, and animated through the same path as
//! everyone (`actors::ActorRig`, picked by `world::animation::pick` as
//! `Actor::PickAnimations`, `00895110`, picks for people): the idle; the
//! walk, run or sneak in the direction the keys move the player (the
//! mover's flags), with the weapon kind's groups while it's drawn; drawing
//! and putting the weapon away (`Equip`/`Unequip`, the weapon in hand at
//! their `Attach`/`Detach` keys); the aim and attacks while it's out;
//! sitting down, the seated loop and getting up from the furniture
//! procedure (`world::furniture::Sitter`), switched without a blend where
//! the procedure turns the player (`cSkipNextBlend`, as for people).
//!
//! Shown while the game shows the third-person body
//! (`PlayerCamera::actually_third`, `00951a10`); the first-person view
//! (`viewmodel`) shows otherwise.
//!
//! Not yet (left out, not substituted): jumping's groups (`mtjump*.kf`),
//! turning in place (the player's turn flags aren't kept), the face's
//! blinking and lip movement, the body fading when the camera is inside
//! it, the dead player's ragdoll.

use bevy::prelude::*;
use cellview::space;
use esm::FormId;
use world::animation::{group, MoveFlags};
use world::dialogue::PLAYER_REF;

use crate::actors::ActorRig;
use crate::walk::Player;
use crate::{FlyCamera, GameFiles, Spawner};

/// What the body is built from, to know when to rebuild it.
#[derive(Clone, PartialEq)]
struct Built {
    weapon: Option<FormId>,
    worn: Vec<FormId>,
    female: bool,
}

/// The player's third-person body on screen.
#[derive(Resource, Default)]
pub struct PlayerBody {
    built: Option<Built>,
    /// The place's lighting it's lit with (`PlaceLighting::changes`), and
    /// its pieces, to light them again when that changes.
    lit_at: u64,
    lit: crate::LitPieces,
    /// The entity placing it where the player stands, and the skeleton's
    /// root (which carries the `ActorRig`).
    holder: Option<Entity>,
    root: Option<Entity>,
    scale: f32,
}

/// A skeleton's bones by name (two bodies with the same can share their
/// animations' state).
fn bone_names(bones: &[nif::Bone]) -> Vec<String> {
    bones.iter().map(|b| b.name.to_ascii_lowercase()).collect()
}

impl PlayerBody {
    /// The skeleton's root entity (carrying its `ActorRig`), once built.
    pub fn root(&self) -> Option<Entity> {
        self.root
    }
}

/// The holder's placement: at the feet, turned to the heading (clockwise
/// from north), at the actor's scale.
fn placement(feet: [f32; 3], heading: f32, scale: f32) -> Transform {
    let (sh, ch) = heading.sin_cos();
    let m = [
        ch * scale,
        -sh * scale,
        0.0,
        0.0,
        sh * scale,
        ch * scale,
        0.0,
        0.0,
        0.0,
        0.0,
        scale,
        0.0,
        feet[0],
        feet[1],
        feet[2],
        1.0,
    ];
    Transform::from_matrix(Mat4::from_cols_array(&space::matrix(&m)))
}

/// Keeps the body built for what the player is and wears, placed and told
/// what to play; `actors::animate_actors` poses it after.
#[allow(clippy::too_many_arguments)]
pub fn update_player_body(
    mut commands: Commands,
    game: Res<GameFiles>,
    state: Res<crate::dialogue::DialogueState>,
    player: Res<Player>,
    view: Res<crate::player_camera::PlayerView>,
    attack: Res<crate::combat::PlayerAttack>,
    mut seats: ResMut<crate::sitting::Seats>,
    mut seat: ResMut<crate::sitting::PlayerSeat>,
    mut body: ResMut<PlayerBody>,
    mut spawner: Spawner,
    cameras: Query<&FlyCamera>,
    mut placed: Query<(&mut Transform, &mut Visibility)>,
    mut rigs: Query<&mut ActorRig>,
    mut library: Option<ResMut<crate::anim_library::AnimLibrary>>,
) {
    let order = &game.0.order;
    let st = &state.0;
    let Some(lighting) = spawner.place_lighting.get() else {
        return;
    };
    let weapon = world::combat::weapon_in_hand(order, st, PLAYER_REF);
    let worn: Vec<FormId> = st
        .equipped
        .get(&PLAYER_REF)
        .into_iter()
        .flatten()
        .copied()
        .filter(|&i| {
            order
                .get(i)
                .is_some_and(|r| r.entry.header.kind.as_bytes() == b"ARMO")
        })
        .collect();
    let wanted = Built {
        weapon: weapon.as_ref().map(|w| w.form_id),
        worn,
        female: st.player_female.unwrap_or(false),
    };
    let lighting_changes = spawner.place_lighting.changes();
    if body.built.as_ref() == Some(&wanted)
        && body.lit_at != lighting_changes
        && body.root.is_some()
    {
        // The place's lighting changed (another place, or outdoors another
        // square loaded): the same materials made with it, as rebuilding
        // the body would make them, without rebuilding it.
        body.lit_at = lighting_changes;
        spawner.relight_lone_actor(&body.lit, lighting);
    }
    if body.built.as_ref() != Some(&wanted) || body.lit_at != lighting_changes {
        // The animations playing go on in the new body (it's rebuilt for
        // another weapon or clothes): only the model changes, as in the
        // game.
        let carried = body.root.and_then(|r| rigs.get(r).ok()).map(|r| {
            (
                r.player.clone(),
                r.picker.clone(),
                bone_names(&r.skeleton.bones),
                r.weapon_parent.clone(),
            )
        });
        let weapon_changed = body.built.as_ref().map(|b| b.weapon) != Some(wanted.weapon);
        if let Some(old) = body.holder.take() {
            if let Ok(mut e) = commands.get_entity(old) {
                e.despawn();
            }
        }
        body.root = None;
        body.lit = crate::LitPieces::default();
        let look = world::actor::player_look(order, wanted.female, &wanted.worn, wanted.weapon);
        body.built = Some(wanted);
        body.lit_at = lighting_changes;
        let Some(look) = look else {
            return;
        };
        let scene = game.0.actor_scene(&look);
        let holder = commands
            .spawn((Transform::IDENTITY, Visibility::Hidden))
            .id();
        body.holder = Some(holder);
        let Some((root, joints, _, lit)) =
            spawner.spawn_lone_actor_lit(&scene, lighting, holder, 0)
        else {
            return;
        };
        body.lit = lit;
        let skeleton = scene.actors[0].skeleton.clone();
        // The player's animations from the start (the game's from the
        // load): the damage-a-second figures read their attack keys
        // (`PlayerCharacter::GetAnimation(0)`, `world::dps`).
        if let Some(lib) = library.as_deref_mut() {
            lib.player = Some(lib.set_for(&game.0, &skeleton));
        }
        body.scale = look.scale;
        let mut rig = ActorRig::new(skeleton, look.scale, 0.0);
        rig.joints = joints;
        // A rebuilt body carries on what played; a first one starts as the
        // weapon is: drawn or not.
        match carried {
            Some((player, picker, names, parent)) if names == bone_names(&rig.skeleton.bones) => {
                rig.player = player;
                rig.picker = picker;
                rig.weapon_parent = parent;
            }
            _ => rig.picker.drawn = attack.out,
        }
        // Another weapon in hand: equipping or unequipping one sets the
        // weapon put away (`0088db20` / `0088d7d0`: `SetWeaponDrawn(0)`,
        // process vfunc +0x458; `combat` has it wanted away too), unless
        // `--weapon` drew it at once; then its model is attached and the
        // process puts it where that state has it (`004ab750` →
        // `ForceWeaponDrawnSheathed`, `Picker::weapon_attached`): before,
        // the old weapon's aim (the fists' raised guard) went on playing
        // with the new weapon, and its `Weapon` track, made for the
        // forearm's twist bone, put the gun in front of the body.
        if weapon_changed {
            rig.picker.drawn = attack.out;
            rig.weapon_attached = weapon.is_some();
        }
        rig.want_drawn = attack.out;
        rig.character = true;
        rig.weapon_kind = weapon
            .as_ref()
            .map(|w| world::animation::groups::weapon_kind(w.animation));
        commands.entity(root).insert(rig);
        body.root = Some(root);
        return;
    }
    let (Some(holder), Some(root)) = (body.holder, body.root) else {
        return;
    };
    let shown = view.camera.actually_third && player.ready;
    let sitter = st.sitters.get(&PLAYER_REF);
    let heading = match (sitter, cameras.single()) {
        (Some(s), _) => s.heading,
        (None, Ok(fly)) => (-fly.yaw).rem_euclid(std::f32::consts::TAU),
        (None, Err(_)) => 0.0,
    };
    if let Ok((mut transform, mut visibility)) = placed.get_mut(holder) {
        let want = if shown {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
        if *visibility != want {
            *visibility = want;
        }
        *transform = placement(player.character.feet, heading, body.scale);
    }
    let Ok(mut rig) = rigs.get_mut(root) else {
        return;
    };
    // The keys' movement (the mover's flags), sneaking (0x400), and the
    // weapon: wanted out as the Ready Item key and attacking have it (the
    // process's `GetWantWeaponDrawn`), its kind and attack group.
    let moving = player.speed > 0.0 && sitter.is_none();
    rig.walking = moving;
    rig.running = moving && player.moving.running;
    rig.sneaking = st.player_sneaking;
    rig.speed = if moving {
        player.speed * body.scale
    } else {
        0.0
    };
    rig.direction = moving.then_some(MoveFlags {
        turn_left: false,
        turn_right: false,
        ..player.moving
    });
    rig.want_drawn = attack.out;
    rig.fighting = false;
    rig.character = true;
    rig.seated = sitter.is_some();
    rig.weapon_kind = weapon
        .as_ref()
        .map(|w| world::animation::groups::weapon_kind(w.animation));
    rig.attack_group = match weapon.as_ref().map(|w| w.attack_animation) {
        Some(g @ 26..=0xa8) => g,
        _ => group::ATTACK_RIGHT,
    };
    rig.attack_at = attack.fired_at;
    // The reload under way: the weapon's reload group (`ReloadA` …
    // `ReloadZ`, as for people). Before, the third-person body never
    // played it.
    rig.reload_at = match (attack.reload_started, weapon.as_ref()) {
        (Some(at), Some(w)) => Some((at, group::RELOAD_A + w.reload_animation.min(22))),
        _ => None,
    };
    // The procedure turned the player: no blend (`cSkipNextBlend`).
    if std::mem::take(&mut seat.skip_next_blend) {
        rig.player.skip_next_blend();
    }
    // Furniture: the entry or exit over everything, the seat's loop
    // under it (as `sitting::furniture_frame` gives people).
    match sitter {
        Some(s) => {
            rig.dynamic_idle = s
                .dynamic_idle
                .as_ref()
                .and_then(|(_, model)| seats.sequence(&game.0, model));
            rig.overlay = s
                .playing
                .as_ref()
                .and_then(|p| Some((seats.sequence(&game.0, &p.model)?, p.elapsed)));
        }
        None => {
            rig.dynamic_idle = None;
            rig.overlay = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_body_stands_at_the_feet_facing_the_heading() {
        // Heading east: the body's forward (its y) points east.
        let t = placement([100.0, 200.0, 10.0], std::f32::consts::FRAC_PI_2, 1.0);
        let forward = t.rotation * Vec3::Y;
        let east = Vec3::from(space::direction([1.0, 0.0, 0.0]));
        assert!(forward.abs_diff_eq(east, 1e-5), "{forward} vs {east}");
        let at = Vec3::from(space::point([100.0, 200.0, 10.0]));
        assert!(t.translation.abs_diff_eq(at, 1e-5));
    }
}

//! What the crosshair is on, once a frame: the game's view caster
//! (`physics::view_caster`, `0070bc20` → `00631d60`) looking along the
//! eye's view into the cell's collision (every placed reference's shapes,
//! moving bodies where they are now) and the people: a living one's
//! character controller (the capsule every actor's controller is,
//! `physics::CharacterShape::PLAYER`, a phantom on layer 30), a dead one's
//! ragdoll bones (`actors::DeadBody`, on their own layers).
//!
//! Everything the crosshair offers reads this one pick: the person to talk
//! to (`dialogue::talk`), the load door or door that swings (`walk`), the
//! object E uses (`scripts`), the Info panel (`hud`) and the Grab key
//! (`clutter`).
//!
//! What stands in for the game's here (see `physics::view_caster`): the
//! exact pick on a person is the ray against their controller's capsule,
//! and on anything else against its collision triangles (the game's
//! Gamebryo pick uses the drawn triangles); a dead actor without a ragdoll
//! offers no shape at all.

use bevy::prelude::*;
use esm::FormId;
use physics::view_caster::{self, Capsule, Pick, World};
use world::activation::{PICK_LENGTH, PICK_RADIUS};
use world::dialogue::PLAYER_REF;

use crate::actors::ActorRig;
use crate::ai::Walker;
use crate::dialogue::{DialogueState, Talkers};
use crate::walk::{game_point, CellCollision};
use crate::{FlyCamera, GameFiles};

/// The view caster's pick this frame.
#[derive(Resource, Default)]
pub struct Crosshair(pub Option<Pick>);

impl Crosshair {
    /// The reference E would use: the pick within `iActivatePickLength`
    /// (`0070bc20` +0xfc).
    pub fn target(&self) -> Option<FormId> {
        self.0
            .filter(|p| view_caster::within(p.distance, PICK_LENGTH))
            .map(|p| FormId(p.reference))
    }
}

/// The people's shapes for the pick: the living's controllers, the dead's
/// ragdoll capsules.
pub(crate) fn people_shapes(
    talkers: &Talkers,
    state: &world::scripting::GameState,
    rigs: &Query<(&Walker, &ActorRig)>,
) -> Vec<Capsule> {
    let shape = physics::CharacterShape::PLAYER;
    let mut out = Vec::new();
    for t in &talkers.0 {
        let r = t.reference.0;
        if state.dead.contains(&t.reference) {
            // A body its critical stage ended (`008a1a70`: stages 2 and 4)
            // has its collision taken out of the Havok world
            // (`0057b520(0)`) before its 3D is culled (`00450f90(1)`):
            // nothing left for the pick.
            if world::more_functions::body_gone(state, t.reference) {
                continue;
            }
            let Some(dead) = rigs
                .iter()
                .find(|(w, _)| w.reference == t.reference)
                .and_then(|(_, rig)| rig.ragdoll.as_deref())
            else {
                continue;
            };
            for (i, a, b, radius) in dead.sim.world_capsules() {
                out.push(Capsule {
                    reference: r,
                    body: Some(u64::from(r) << 8 | i as u64),
                    layer: dead.sim.bodies[i].layer,
                    a,
                    b,
                    radius,
                });
            }
            continue;
        }
        let [x, y, z] = t.position;
        let bottom = z + shape.lift + shape.radius;
        let top = (z + shape.lift + shape.height - shape.radius).max(bottom);
        out.push(Capsule {
            reference: r,
            body: None,
            layer: physics::layers::layer::CHAR_CONTROLLER,
            a: [x, y, bottom],
            b: [x, y, top],
            radius: shape.radius,
        });
    }
    out
}

/// `NV_LOG_CROSSHAIR` set: each change of the crosshair's reference is
/// logged (for checking the pick in a running viewer).
fn logging() -> bool {
    static ON: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("NV_LOG_CROSSHAIR").is_some())
}

/// The pick from the eye along its view (the camera is the eye during the
/// frame, `player_camera`).
pub fn pick(
    game: Res<GameFiles>,
    state: Res<DialogueState>,
    collision: Res<CellCollision>,
    talkers: Res<Talkers>,
    rigs: Query<(&Walker, &ActorRig)>,
    cameras: Query<&Transform, With<FlyCamera>>,
    mut crosshair: ResMut<Crosshair>,
) {
    let Ok(camera) = cameras.single() else {
        crosshair.0 = None;
        return;
    };
    let order = &game.0.order;
    let eye = game_point(camera.translation);
    let f = camera.forward().as_vec3();
    let capsules = people_shapes(&talkers, &state.0, &rigs);
    let world = World {
        collider: &collision.0,
        capsules: &capsules,
        origin: eye,
        direction: [f.x, -f.z, f.y],
        fuzzy_replaces: |r| world::activation::fuzzy_replaces(order, FormId(r)),
        // Sitting or sleeping people's controllers: not applied (see
        // `physics::view_caster`).
        excluded: |_| false,
    };
    let now = world.pick(
        PICK_LENGTH,
        PICK_RADIUS,
        PLAYER_REF.0,
        &view_caster::Settings::default(),
    );
    if now.map(|p| p.reference) != crosshair.0.map(|p| p.reference) && logging() {
        match now {
            Some(p) => println!(
                "Crosshair: {} at {:.0}{}.",
                FormId(p.reference),
                p.distance,
                if p.fuzzy { " (fuzzy)" } else { "" }
            ),
            None => println!("Crosshair: nothing."),
        }
    }
    crosshair.0 = now;
}

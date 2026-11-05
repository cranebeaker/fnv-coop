//! Scene model glue (see docs/COOP-SCENES.md): every peer simulates the
//! scene around its own player, so these systems only (1) tell the session
//! which exterior cells exist, so it can work out which cells each peer has
//! loaded, and (2) move the local player when the host places it (late join,
//! party warp, tether warp to a point).

use std::collections::HashSet;

use bevy::prelude::*;
use esm::FormId;
use world::scripting::GameState;

use super::LiveNow;
use crate::coop::CoopLink;
use crate::dialogue::DialogueState;
use crate::GameFiles;

/// Registers the grid of the worldspace the local player is in, once per
/// worldspace, in the session's cell table.
pub fn register_grid(
    mut link: NonSendMut<CoopLink>,
    state: Res<DialogueState>,
    game: Res<GameFiles>,
    mut done: Local<HashSet<u32>>,
) {
    let Some(w) = state.0.player_world else {
        return;
    };
    if done.contains(&w.0) {
        return;
    }
    let Some(graph) = link.0.live_roam_graph() else {
        return;
    };
    match world::WorldGrid::load(&game.0.order, w) {
        Ok(grid) => {
            world::coop::roam_ops::register_world_grid(graph, &grid);
            done.insert(w.0);
        }
        Err(e) => {
            println!("Co-op: cannot register the worldspace grid: {e}");
            done.insert(w.0);
        }
    }
}

/// How far above the feet the floor test starts, and how far from the feet
/// the floor may be (game units).
const FLOOR_ABOVE: f32 = 60.0;
const FLOOR_NEAR: f32 = 40.0;

/// True when the cell's collision has a floor near `feet`: a ray down from a
/// little above the feet hits a surface within [`FLOOR_NEAR`] of them. The
/// host places a guest on a spot beside a player without the geometry, and
/// that spot can be past a wall or over nothing.
pub(crate) fn floor_under(collision: &physics::Collider, feet: [f32; 3]) -> bool {
    let from = [feet[0], feet[1], feet[2] + FLOOR_ABOVE];
    collision
        .raycast(from, [0.0, 0.0, -1.0], FLOOR_ABOVE + FLOOR_NEAR)
        .is_some_and(|(d, _)| (from[2] - d - feet[2]).abs() <= FLOOR_NEAR)
}

/// An interior start at `feet` facing `heading` when the cell has a floor there.
/// Else the scene keeps its arrival point (the player then does not fall out
/// of the world): `None`.
pub(crate) fn start_on_floor(
    scene: &cellview::ViewerScene,
    feet: [f32; 3],
    heading: f32,
    via: &'static str,
) -> Option<cellview::Start> {
    if !floor_under(&scene.collision, feet) {
        println!(
            "Co-op: no floor at {:.0}, {:.0}, {:.0} in {}; starting at the arrival point.",
            feet[0], feet[1], feet[2], scene.cell
        );
        return None;
    }
    Some(cellview::Start {
        eye: [feet[0], feet[1], feet[2] + cellview::EYE_HEIGHT],
        heading,
        via,
    })
}

/// Moves the player to a placement the host sent (once), the way a script's
/// `MoveTo` does (`scripts::move_player`) but to a cell and a position.
pub fn apply_place(
    mut now: ResMut<LiveNow>,
    state: Res<DialogueState>,
    game: Res<GameFiles>,
    mut pending: ResMut<crate::PendingScene>,
    mut pending_exterior: ResMut<crate::exterior::PendingExterior>,
) {
    if pending.0.is_some() || pending_exterior.0.is_some() {
        return;
    }
    let Some(place) = now.view.as_mut().and_then(|v| v.roam.place.take()) else {
        return;
    };
    let gs: &GameState = &state.0;
    let order = &game.0.order;
    let cell = FormId(place.cell.0);
    let Some(rr) = order.get(cell) else {
        println!("Co-op: the host placed us in {cell}, which is not in the load order.");
        return;
    };
    let [x, y, z] = place.pos;
    match order.world_of(&rr) {
        Some(world) => match world::WorldGrid::load(order, world) {
            Ok(grid) => {
                pending_exterior.0 = Some(crate::exterior::ExteriorStart {
                    grid,
                    feet: [x, y],
                    height: Some(z),
                    heading: place.heading,
                });
            }
            Err(e) => println!("Co-op: cannot enter the host's worldspace: {e}"),
        },
        None => match game.0.load_cell_now(cell, &gs.disabled) {
            Ok(mut scene) => {
                if let Some(start) = start_on_floor(
                    &scene,
                    [x, y, z],
                    place.heading,
                    "the host (co-op placement)",
                ) {
                    scene.start = start;
                }
                pending.0 = Some(scene);
            }
            Err(e) => println!("Co-op: cannot enter the host's cell: {}", e.0),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A floor square from (0, 0) to (100, 100) at height 10.
    fn floor() -> physics::Collider {
        let mut c = physics::Collider::new();
        let v = [
            [0.0, 0.0, 10.0],
            [100.0, 0.0, 10.0],
            [100.0, 100.0, 10.0],
            [0.0, 100.0, 10.0],
        ];
        c.add(&v, &[[0, 1, 2], [0, 2, 3]]);
        c
    }

    #[test]
    fn a_spot_needs_a_floor_near_its_feet() {
        let c = floor();
        assert!(floor_under(&c, [50.0, 50.0, 10.0]), "on the floor");
        assert!(floor_under(&c, [50.0, 50.0, 30.0]), "a little above it");
        assert!(
            !floor_under(&c, [150.0, 50.0, 10.0]),
            "past the floor's edge"
        );
        assert!(!floor_under(&c, [50.0, 50.0, 200.0]), "high above it");
        assert!(!floor_under(&c, [50.0, 50.0, -100.0]), "under it");
    }
}

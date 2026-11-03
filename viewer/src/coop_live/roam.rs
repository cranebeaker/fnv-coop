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
                scene.start = cellview::Start {
                    eye: [x, y, z + cellview::EYE_HEIGHT],
                    heading: place.heading,
                    via: "the host (co-op placement)",
                };
                pending.0 = Some(scene);
            }
            Err(e) => println!("Co-op: cannot enter the host's cell: {}", e.0),
        },
    }
}

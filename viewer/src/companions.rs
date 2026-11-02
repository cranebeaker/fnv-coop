//! Followers and teammates coming along with the player
//! (`world::companions::come_along`): when the player has been put
//! somewhere — another place (a load door, `coc`), or by a script's
//! `MoveTo` or fast travel (`GameState::player_placed`) — those running
//! near them the frame before (the people drawn here, the game's
//! high-process list) who are the player's teammates, or who follow the
//! player and are now far, are moved to the player. Those moved into the
//! new place come on screen as anyone moved there does
//! (`bring_in_people`); those already here start again from there
//! (`ai::Moved`).

use bevy::prelude::*;
use esm::FormId;

/// The plugin.
pub struct CompanionsPlugin;

impl Plugin for CompanionsPlugin {
    fn build(&self, app: &mut App) {
        // With the AI work (stage 6, `crate::frame_order`), placed for order
        // only ahead of the threads' start: its own place in the frame isn't
        // traced.
        app.init_resource::<Along>().add_systems(
            Update,
            come_along
                .in_set(crate::frame_order::FrameSet::Stage(
                    world::frame::Stage::AiStart,
                ))
                .before(crate::frame_order::FrameSet::step(
                    crate::frame_order::START_THREADS,
                ))
                .before(crate::frame_order::AiSet::Call(
                    crate::frame_order::INTERFACE_IDLE,
                )),
        );
    }
}

/// The player's place last frame and who was loaded near them.
#[derive(Resource, Default)]
struct Along {
    space: Option<FormId>,
    near: Vec<FormId>,
}

fn come_along(
    game: Res<crate::GameFiles>,
    scripts: Res<crate::scripts::Scripts>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    walkers: Query<(&crate::ai::Walker, &Visibility)>,
    player: Res<crate::walk::Player>,
    mut moved: ResMut<crate::ai::Moved>,
    mut along: ResMut<Along>,
) {
    if !player.ready {
        return;
    }
    let state = &mut state.0;
    let here = state.player_world.or(state.player_cell);
    let placed = std::mem::take(&mut state.player_placed);
    if (placed || here != along.space) && along.space.is_some() && here.is_some() {
        let near = std::mem::take(&mut along.near);
        let came = world::companions::come_along(&game.0.order, &scripts.0, state, &near);
        for who in &came {
            println!("{who} comes along with the player.");
        }
        moved.0.extend(came);
    }
    along.space = here;
    along.near = walkers
        .iter()
        .filter(|(_, v)| **v != Visibility::Hidden)
        .map(|(w, _)| w.reference)
        .collect();
}

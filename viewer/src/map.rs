//! The world map's markers (`world::map`): found as the player walks, and
//! listed on the Pip-Boy screen's Map tab to travel to.

use bevy::prelude::*;
use esm::FormId;

use crate::dialogue::DialogueState;
use crate::GameFiles;

/// The map markers of the worldspace the player is in.
#[derive(Resource, Default)]
pub struct MapMarkers {
    pub world: Option<FormId>,
    pub list: Vec<world::map::MapMarker>,
}

/// Once a second outdoors: the worldspace's markers (read once per
/// worldspace), and any the player has now come near enough to find,
/// which the game announces.
pub fn find_markers(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    exterior: Option<Res<crate::exterior::Exterior>>,
    mut markers: ResMut<MapMarkers>,
    mut last: Local<f32>,
) {
    let now = time.elapsed_secs();
    if now - *last < 1.0 {
        return;
    }
    *last = now;
    let Some(exterior) = exterior else {
        return;
    };
    let world = exterior.grid.world.form_id;
    if markers.world != Some(world) {
        markers.world = Some(world);
        markers.list = exterior
            .grid
            .persistent
            .map(|p| world::map::markers(&game.0.order, p))
            .unwrap_or_default();
        println!("  {} map markers", markers.list.len());
    }
    let state = &mut state.0;
    let Some(feet) = state.player_position else {
        return;
    };
    for m in world::map::discover(&game.0.order, state, &markers.list, feet) {
        println!("Found {} on the map.", m.name);
    }
}

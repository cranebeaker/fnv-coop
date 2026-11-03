//! Player-world interactions in the viewer (thin glue over
//! `world::coop::interact`): following the party through a load door, and
//! telling the player when the host refused something.
//!
//! The rules live in the world crate. A game site asks with
//! `GameState::coop_route(act)` (host and single player run the rule
//! themselves; a guest only asks) and this module reports what came back.

use bevy::prelude::*;
use esm::FormId;
use world::coop::interact::arrival_of;
use world::coop::Authority;

use crate::dialogue::DialogueState;
use crate::hud::{HudMessage, HudMessages};
use crate::scripts::Notices;
use crate::GameFiles;

/// The words for a refusal.
fn refusal(why: coop_session::interact::Why) -> &'static str {
    use coop_session::interact::Why;
    match why {
        Why::None => "",
        Why::Unknown => "That is not possible here.",
        Why::Gone => "It is gone.",
        Why::Insufficient => "You do not have enough.",
        Why::Locked => "It is locked.",
        Why::Policy => "The party has to stay together.",
        Why::Invalid => "That is not possible.",
        Why::Busy => "Someone else is using it.",
    }
}

/// Loads the destination of a door the party went through (the host's
/// answer to a guest's own door, or the leader's door a guest follows).
#[allow(clippy::too_many_arguments)]
pub fn follow_door(
    mut state: ResMut<DialogueState>,
    mut now: ResMut<super::LiveNow>,
    game: Res<GameFiles>,
    mut pending: ResMut<crate::PendingScene>,
    mut pending_exterior: ResMut<crate::exterior::PendingExterior>,
) {
    let gs = &mut state.0;
    if gs.coop.authority == Authority::Single || gs.coop.interact.follow.is_none() {
        return;
    }
    if pending.0.is_some() || pending_exterior.0.is_some() || gs.player_cell.is_none() {
        return;
    }
    let Some(f) = gs.coop.interact.follow.take() else {
        return;
    };
    let order = &game.0.order;
    let Some(a) = arrival_of(order, gs, FormId(f.door.0)) else {
        println!("Co-op: can't find where door {} leads.", f.door.0);
        return;
    };
    world::script_functions::player_moved(gs);
    now.counters.doors_followed += 1;
    if !a.interior {
        match world::WorldGrid::load(order, a.space) {
            Ok(grid) => {
                println!("Co-op: following through a door outdoors.");
                pending_exterior.0 = Some(crate::exterior::ExteriorStart {
                    grid,
                    feet: [a.pos[0], a.pos[1]],
                    height: Some(a.pos[2]),
                    heading: a.heading,
                });
            }
            Err(e) => println!("Co-op: can't follow through the door: {e}"),
        }
        return;
    }
    match game.0.load_cell_now(a.cell, &gs.disabled) {
        Ok(mut scene) => {
            println!("Co-op: following through a door into {}.", a.cell);
            scene.start = cellview::Start {
                eye: [a.pos[0], a.pos[1], a.pos[2] + cellview::EYE_HEIGHT],
                heading: a.heading,
                via: "the party (co-op)",
            };
            pending.0 = Some(scene);
        }
        Err(e) => println!("Co-op: can't follow through the door: {}", e.0),
    }
}

/// Shows what the host refused.
pub fn results(
    time: Res<Time>,
    mut state: ResMut<DialogueState>,
    (mut messages, mut notices): (ResMut<HudMessages>, ResMut<Notices>),
) {
    let gs = &mut state.0;
    if gs.coop.interact.results.is_empty() {
        return;
    }
    for r in std::mem::take(&mut gs.coop.interact.results) {
        if r.ok {
            continue;
        }
        let text = refusal(r.why).to_string();
        if messages.on {
            messages.queue.push(HudMessage::from(text));
        } else {
            notices.push(text, time.elapsed_secs());
        }
    }
}

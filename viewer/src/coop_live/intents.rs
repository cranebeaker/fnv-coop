//! What the local player does that the whole party must see: containers,
//! dialogue, fast travel, and following the host to a new place.

use std::collections::VecDeque;

use bevy::prelude::*;
use coop_session::{Intent, Role};
use esm::FormId;
use world::coop::apply::Ctx;
use world::coop::facts::LootOp;

use super::LiveNow;
use crate::coop::CoopLink;
use crate::dialogue::{Conversation, DialogueState};
use crate::hud::{HudMessage, HudMessages};
use crate::scripts::{Notices, Scripts};
use crate::GameFiles;

/// Seconds an intent waits for the session before it is dropped.
const WAIT: f32 = 20.0;
/// Seconds a spectator takes to read a line before it reports it done.
const READ_SECONDS: f32 = 2.5;

/// Container actions the menus queued, carried to the party. The host
/// registers a container the first time it opens it.
pub fn loot(
    mut link: NonSendMut<CoopLink>,
    mut state: ResMut<DialogueState>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
) {
    let gs = &mut state.0;
    if gs.coop.live.loot_ops.is_empty() {
        return;
    }
    let cx = Ctx {
        order: &game.0.order,
        scripts: &scripts.0,
    };
    link.0.live_loot_ops(cx, gs);
}

/// What the dialogue menu did, waiting for the session to take it.
#[derive(Resource, Default)]
pub struct Tap {
    speaker: Option<FormId>,
    info: Option<FormId>,
    queue: VecDeque<(Intent, f32)>,
}

/// The viewer's own dialogue menu, watched from outside: opening it opens a
/// session, each line said is a pick, a line finished is `line_done`,
/// closing it closes the session. One intent a frame, each when the session
/// can take it.
pub fn tap_dialogue(
    time: Res<Time>,
    conversation: Res<Conversation>,
    now: Res<LiveNow>,
    mut link: NonSendMut<CoopLink>,
    mut tap: ResMut<Tap>,
) {
    let t = time.elapsed_secs();
    let talk = conversation.0.as_ref().filter(|x| !x.is_line_only());
    match (tap.speaker, talk) {
        (None, Some(x)) => {
            let s = x.speaker();
            tap.speaker = Some(s);
            tap.info = None;
            tap.queue.push_back((Intent::DialogueOpen(s), t));
        }
        (Some(_), None) => {
            if tap.info.take().is_some() {
                tap.queue.push_back((Intent::DialogueLineDone, t));
            }
            tap.speaker = None;
            tap.queue.push_back((Intent::DialogueClose, t));
        }
        _ => {}
    }
    if let Some(x) = talk {
        let said = x.said().map(|((info, _, _), _)| info);
        if said != tap.info {
            if tap.info.is_some() {
                tap.queue.push_back((Intent::DialogueLineDone, t));
            }
            if let Some(i) = said {
                tap.queue.push_back((Intent::DialoguePick(i), t));
            }
            tap.info = said;
        }
    }
    let d = now.view.as_ref().and_then(|v| v.dialogue.as_ref());
    let Some((intent, since)) = tap.queue.front().cloned() else {
        return;
    };
    let ready = match &intent {
        Intent::DialogueOpen(_) => link.0.is_ready(),
        Intent::DialoguePick(_) => d.is_some_and(|d| d.mine && d.playing.is_none()),
        Intent::DialogueLineDone => d.is_some_and(|d| d.playing.is_some()),
        _ => d.is_some_and(|d| d.mine),
    };
    if ready {
        link.0.live_intent(intent);
        tap.queue.pop_front();
    } else if t - since > WAIT {
        tap.queue.pop_front();
    }
}

/// Lines other players picked are shown as subtitles and reported done
/// after a reading time; the picker's own menu plays theirs.
#[derive(Resource, Default)]
pub struct Lines {
    due: Vec<f32>,
}

/// Plays the picked lines of the session.
pub fn play_lines(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut now: ResMut<LiveNow>,
    mut link: NonSendMut<CoopLink>,
    mut lines: ResMut<Lines>,
    (mut messages, mut notices): (ResMut<HudMessages>, ResMut<Notices>),
) {
    let t = time.elapsed_secs();
    let Some(view) = now.view.clone() else { return };
    let mine = view.dialogue.as_ref().is_some_and(|d| d.mine);
    for (_, info) in &view.lines {
        now.counters.lines_played += 1;
        if mine {
            continue;
        }
        let text = world::dialogue::Info::load(&game.0.order, *info)
            .and_then(|i| i.responses.first().map(|r| r.text.clone()))
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| format!("(line {info})"));
        if messages.on {
            messages.queue.push(HudMessage::from(text));
        } else {
            notices.push(text, t);
        }
        lines.due.push(t + READ_SECONDS);
    }
    if view.dialogue.is_none() {
        lines.due.clear();
        return;
    }
    let before = lines.due.len();
    lines.due.retain(|at| *at > t);
    for _ in lines.due.len()..before {
        link.0.live_intent(Intent::DialogueLineDone);
    }
}

/// A guest starts where the host is: the guest's scene is replaced once, at
/// the start (as a script's `MoveTo` does, `scripts::move_player`). It does
/// not follow the host afterwards: players roam free.
#[allow(clippy::too_many_arguments)]
pub fn follow_place(
    link: NonSend<CoopLink>,
    state: Res<DialogueState>,
    game: Res<GameFiles>,
    mut pending: ResMut<crate::PendingScene>,
    mut pending_exterior: ResMut<crate::exterior::PendingExterior>,
    mut last: Local<Option<(u32, u32)>>,
) {
    if link.0.role() != Role::Guest || !link.0.is_ready() {
        return;
    }
    let Some(place) = link.0.host_place() else {
        return;
    };
    // making its own character in a local room: not yet (`coop_session::start`)
    if !link.0.start_follow_ok() {
        return;
    }
    let gs = &state.0;
    if gs.player_cell.is_none() || pending.0.is_some() || pending_exterior.0.is_some() {
        return;
    }
    let key = (place.cell, place.world);
    // free roam (docs/COOP-SCENES.md): the guest joins the host's scene once, at
    // the start; after that every peer simulates the scene around its own player
    if last.is_some() {
        return;
    }
    *last = Some(key);
    let there = if place.world != 0 {
        gs.player_world.map(|w| w.0) == Some(place.world)
    } else {
        gs.player_world.is_none() && gs.player_cell.map(|c| c.0) == Some(place.cell)
    };
    let Some(pos) = place.pos.filter(|_| !there) else {
        return;
    };
    let order = &game.0.order;
    let space = FormId(if place.world != 0 {
        place.world
    } else {
        place.cell
    });
    let interior = order
        .get(space)
        .is_some_and(|r| r.entry.header.kind == esm::sig::CELL);
    if !interior {
        match world::WorldGrid::load(order, space) {
            Ok(grid) => {
                println!("Co-op: following the host outdoors.");
                pending_exterior.0 = Some(crate::exterior::ExteriorStart {
                    grid,
                    feet: [pos[0], pos[1]],
                    height: Some(pos[2]),
                    heading: 0.0,
                });
            }
            Err(e) => println!("Co-op: can't follow the host: {e}"),
        }
        return;
    }
    match game.0.load_cell_now(space, &gs.disabled) {
        Ok(mut scene) => {
            println!("Co-op: following the host into {space}.");
            scene.start = cellview::Start {
                eye: [pos[0], pos[1], pos[2] + cellview::EYE_HEIGHT],
                heading: 0.0,
                via: "the host (co-op)",
            };
            pending.0 = Some(scene);
        }
        Err(e) => println!("Co-op: can't follow the host: {}", e.0),
    }
}

/// Fast travel the party allowed is carried out (the trip was refused with
/// "Waiting for the party" when it was asked for).
#[allow(clippy::too_many_arguments)]
pub fn travel(
    time: Res<Time>,
    mut now: ResMut<LiveNow>,
    mut state: ResMut<DialogueState>,
    game: Res<GameFiles>,
    markers: Option<Res<crate::map::MapMarkers>>,
    (mut messages, mut notices): (ResMut<HudMessages>, ResMut<Notices>),
) {
    let Some(marker) = now.view.as_ref().and_then(|v| v.flags.travel_allowed) else {
        return;
    };
    let found = markers
        .as_ref()
        .and_then(|ms| ms.list.iter().find(|m| m.reference == marker).cloned());
    let m = found.unwrap_or_else(|| world::map::MapMarker {
        reference: marker,
        name: String::new(),
        position: [0.0; 3],
        radius: 0.0,
        flags: 0,
        kind: 0,
        arrival: None,
    });
    match world::map::travel(&game.0.order, &mut state.0, &m) {
        Ok(_) => {
            now.counters.travels += 1;
            println!("Co-op: the party travels to {marker}.");
        }
        Err(why) => {
            if messages.on {
                messages.queue.push(HudMessage::from(why));
            } else {
                notices.push(why, time.elapsed_secs());
            }
        }
    }
}

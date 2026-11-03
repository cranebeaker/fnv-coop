//! The live frame: the local player's pose goes into the `GameState`, the
//! session's live layer runs, and its view is kept for the other systems.

use bevy::prelude::*;
use coop_session::{FLAG_DRAWN, FLAG_SNEAK};
use world::coop::apply::Ctx;
use world::dialogue::PLAYER_REF;

use super::chat::ChatLog;
use super::LiveNow;
use crate::coop::CoopLink;
use crate::dialogue::DialogueState;
use crate::hud::{HudMessage, HudMessages};
use crate::scripts::{Notices, Scripts};
use crate::GameFiles;

/// Animation bits the avatar publishes (`anim`).
pub const ANIM_MOVING: u16 = 1;
/// Running.
pub const ANIM_RUNNING: u16 = 2;
/// Sneaking.
pub const ANIM_SNEAKING: u16 = 4;
/// In a fight stance (weapon out).
pub const ANIM_FIGHTING: u16 = 8;

/// Writes the local player's pose, runs the live frame, stores its view.
#[allow(clippy::too_many_arguments)]
pub fn run(
    time: Res<Time>,
    mut link: NonSendMut<CoopLink>,
    mut state: ResMut<DialogueState>,
    game: Res<GameFiles>,
    scripts: Res<Scripts>,
    player: Res<crate::walk::Player>,
    attack: Res<crate::combat::PlayerAttack>,
    mut now: ResMut<LiveNow>,
    mut chat: ResMut<ChatLog>,
    (mut messages, mut notices): (ResMut<HudMessages>, ResMut<Notices>),
    mut movies: ResMut<crate::movie::Movies>,
) {
    let order = &game.0.order;
    let gs = &mut state.0;
    let t = time.elapsed_secs();
    let moving = player.speed > 1.0;
    let mut anim = 0u16;
    if moving {
        anim |= ANIM_MOVING;
        if player.moving.running {
            anim |= ANIM_RUNNING;
        }
    }
    if player.sneaking {
        anim |= ANIM_SNEAKING;
    }
    if attack.out {
        anim |= ANIM_FIGHTING;
    }
    let mut flags = 0u16;
    if player.sneaking {
        flags |= FLAG_SNEAK;
    }
    if attack.out {
        flags |= FLAG_DRAWN;
    }
    if gs.coop.live.combat.bot_sneak == Some(true) {
        flags |= FLAG_SNEAK;
    }
    gs.coop.live.local_anim = anim;
    gs.coop.live.local_flags = flags;
    gs.coop.live.local_weapon =
        world::combat::weapon_equipped(order, gs, PLAYER_REF).map_or(0, |w| w.form_id.0);
    let cx = Ctx {
        order,
        scripts: &scripts.0,
    };
    let view = link.0.live_frame(gs, cx, time.delta_secs()).cloned();
    now.counters.hits_sent = u64::from(link.0.live_hits_sent());
    now.counters.tracked = link.0.live_tracked() as u64;
    // the glitch detectors over the remote avatars as this machine draws them
    if let Some(v) = view.as_ref() {
        now.glitch.frame(v.now_ms, &v.avatars);
    }
    let counts = now.glitch.counts;
    now.counters.glitch = counts;
    now.counters.full_refused = u64::from(link.0.full_refusals());
    // scenes: the movie the scene layer starts (at its scene time), stops, and reports done
    if let Some(m) = gs.coop.live.scene.movie_start.take() {
        movies.offset_ms = m.offset_ms;
        movies.queue.push_back(world::scripting::Video {
            file: m.movie.file,
            interruptable: m.movie.interruptable,
            mute_audio: m.movie.mute_audio,
            pause_music: m.movie.pause_music,
            letterbox: m.movie.letterbox,
        });
    }
    if std::mem::take(&mut gs.coop.live.scene.movie_stop) {
        movies.stop();
    }
    gs.coop.live.scene.movie_busy = movies.busy();
    if std::mem::take(&mut movies.skip_pressed) {
        gs.coop_scene_skip(true);
    }
    // a wait or sleep the party agreed to has passed (the host ran the hours once)
    if let Some(shown) = gs.coop.live.scene.time_shown.take() {
        let text = if shown.hours == 1 {
            "1 hour passes.".to_string()
        } else {
            format!("{} hours pass.", shown.hours)
        };
        if messages.on {
            messages.queue.push(HudMessage::from(text));
        } else {
            notices.push(text, t);
        }
    }
    if let Some(v) = &view {
        now.counters.puppet_actions += v.actions.len() as u64;
        // our own lines are shown when typed
        for c in v.chat.iter().filter(|c| c.from != Some(v.me)) {
            chat.push(format!("{}: {}", c.name, c.text), t);
            now.counters.chat_in += 1;
        }
        for n in &v.notices {
            if messages.on {
                messages.queue.push(HudMessage::from(n.clone()));
            } else {
                notices.push(n.clone(), t);
            }
        }
    }
    now.view = view;
}

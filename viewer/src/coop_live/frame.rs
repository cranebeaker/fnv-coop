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
    gs.coop.live.local_anim = anim;
    gs.coop.live.local_flags = flags;
    gs.coop.live.local_weapon = world::combat::weapon_in_hand(order, gs, PLAYER_REF).map_or(0, |w| w.form_id.0);
    let cx = Ctx { order, scripts: &scripts.0 };
    let view = link.0.live_frame(gs, cx, time.delta_secs()).cloned();
    now.counters.hits_sent = u64::from(link.0.live_hits_sent());
    if let Some(v) = &view {
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

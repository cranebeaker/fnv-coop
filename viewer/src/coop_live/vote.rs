//! The party prompt (a vote to sleep or wait, a barrier): shown as a HUD
//! message and answered with F8 (yes) or F9 (no).

use bevy::prelude::*;
use coop_session::{Intent, Prompt};

use super::LiveNow;
use crate::coop::CoopLink;
use crate::hud::{HudMessage, HudMessages};
use crate::scripts::Notices;

/// Shows the open prompt once and sends the player's answer.
pub fn answer(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    now: Res<LiveNow>,
    mut link: NonSendMut<CoopLink>,
    mut shown: Local<Option<String>>,
    (mut messages, mut notices): (ResMut<HudMessages>, ResMut<Notices>),
) {
    let Some(prompt) = now.view.as_ref().and_then(|v| v.prompt.clone()) else {
        *shown = None;
        return;
    };
    let text = match &prompt {
        Prompt::Vote { kind, param, .. } => {
            format!("The party votes: {kind:?} ({param}). F8 yes, F9 no.")
        }
        Prompt::Barrier { .. } => "The party moves on together. F8 yes, F9 no.".to_string(),
    };
    if shown.as_deref() != Some(text.as_str()) {
        *shown = Some(text.clone());
        if messages.on {
            messages.queue.push(HudMessage::from(text));
        } else {
            notices.push(text, time.elapsed_secs());
        }
    }
    if keys.just_pressed(KeyCode::F8) {
        link.0.live_intent(Intent::Answer(true));
    } else if keys.just_pressed(KeyCode::F9) {
        link.0.live_intent(Intent::Answer(false));
    }
}

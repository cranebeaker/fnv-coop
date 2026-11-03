//! Chat: Y opens a line to type (movement is held while typing), Enter
//! sends, Escape cancels. The last lines show at the bottom left for a while.

use std::collections::VecDeque;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::ButtonState;
use bevy::prelude::*;
use coop_session::Intent;

use super::LiveNow;
use crate::coop::CoopLink;

/// Lines kept and shown.
const KEEP: usize = 8;
/// Seconds a line stays up.
const SHOW_SECONDS: f32 = 14.0;
/// Longest line typed.
const MAX_LEN: usize = 200;

/// Recent chat lines with the time they arrived.
#[derive(Resource, Default)]
pub struct ChatLog {
    lines: VecDeque<(f32, String)>,
}

impl ChatLog {
    /// Adds a line.
    pub fn push(&mut self, text: String, at: f32) {
        self.lines.push_back((at, text));
        while self.lines.len() > KEEP {
            self.lines.pop_front();
        }
    }
}

/// The line being typed.
#[derive(Resource, Default)]
pub struct Typing {
    on: bool,
    text: String,
    restore_ready: bool,
}

#[derive(Component)]
pub struct ChatText;

/// The chat panel.
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 15.0,
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.95, 0.8)),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(120.0),
            left: Val::Px(8.0),
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.4)),
        Visibility::Hidden,
        ChatText,
    ));
}

/// Shows the recent lines and the line being typed.
pub fn show(
    time: Res<Time>,
    log: Res<ChatLog>,
    typing: Res<Typing>,
    mut panel: Query<(&mut Text, &mut Visibility), With<ChatText>>,
) {
    let t = time.elapsed_secs();
    let mut lines: Vec<String> = log
        .lines
        .iter()
        .filter(|(at, _)| t - at < SHOW_SECONDS)
        .map(|(_, s)| s.clone())
        .collect();
    if typing.on {
        lines.push(format!("> {}_", typing.text));
    }
    let text = lines.join("\n");
    for (mut shown, mut vis) in &mut panel {
        if shown.0 != text {
            shown.0 = text.clone();
        }
        *vis = if text.is_empty() {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

/// Typing: Y starts, Enter sends, Escape cancels.
#[allow(clippy::too_many_arguments)]
pub fn typing(
    time: Res<Time>,
    mut keys: EventReader<KeyboardInput>,
    buttons: Res<ButtonInput<KeyCode>>,
    mut typing: ResMut<Typing>,
    mut log: ResMut<ChatLog>,
    mut player: ResMut<crate::walk::Player>,
    menus: Res<crate::menus::Menus>,
    conversation: Res<crate::dialogue::Conversation>,
    mut link: NonSendMut<CoopLink>,
    now: Res<LiveNow>,
) {
    if !typing.on {
        if buttons.just_pressed(KeyCode::KeyY)
            && !menus.is_open()
            && conversation.0.is_none()
            && now.view.is_some()
        {
            typing.on = true;
            typing.text.clear();
            typing.restore_ready = player.ready;
            player.ready = false;
        }
        keys.clear();
        return;
    }
    let mut done = false;
    let mut send = false;
    for ev in keys.read() {
        if ev.state != ButtonState::Pressed {
            continue;
        }
        match &ev.logical_key {
            Key::Enter => {
                done = true;
                send = true;
            }
            Key::Escape => done = true,
            Key::Backspace => {
                typing.text.pop();
            }
            Key::Space => typing.text.push(' '),
            Key::Character(c) if typing.text.len() < MAX_LEN => typing.text.push_str(c),
            _ => {}
        }
    }
    if done {
        typing.on = false;
        if typing.restore_ready {
            player.ready = true;
        }
        let text = std::mem::take(&mut typing.text);
        if send && text.starts_with('/') {
            // voice commands (/mute, /unmute, /vol, /voice) stay on this machine
            let reply =
                super::voice::command(&text).unwrap_or_else(|| "Unknown command".to_string());
            log.push(reply, time.elapsed_secs());
        } else if send && !text.trim().is_empty() {
            log.push(format!("You: {text}"), time.elapsed_secs());
            link.0.live_intent(Intent::Say(text));
        }
    }
}

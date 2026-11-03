//! Teammates and pings on the HUD: the ping key, the compass, the Pip-Boy
//! and local maps, and the name tags' health bars and speaking dot. The
//! maths is `coop_session::compass` (pure, tested in coop-core); this file
//! reads the camera and the live view and feeds the existing HUD code.
//!
//! * Ping key (default middle mouse, `--coop-ping-key`): a ping where the
//!   crosshair points. With Shift it sets the waypoint, with Ctrl it clears it.
//! * Compass: teammates (blue ticks) and pings (markers) are added to the
//!   HUD's own compass input in `hud::HudState::input`.
//! * Maps: teammates and pings in the player's place are added to the
//!   world map and local map as extra objective marks (`party_points`).
//! * Name tags: `tag_text` builds the label and the health fraction for the
//!   graphical bar (`hud_bars`); the tag turns green while that player is speaking.
//! * Off-screen hints: pins behind the player are listed as "< Ann 40 m" at the
//!   top of the screen.

use std::sync::Mutex;

use bevy::prelude::*;
use coop_session::compass as c;
use coop_session::{Intent, PlayerId};
use ui::hud::{CompassActor, CompassMarker};

use super::LiveNow;
use crate::coop::CoopLink;
use crate::dialogue::DialogueState;
use crate::hud::{HudMessage, HudMessages};

/// Outdoors, how far (game units) a teammate in another square still counts as here.
const REACH: f32 = 2.5 * world::land::CELL_SIZE;

/// An input the player can bind: a key or a mouse button.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Binding {
    Key(KeyCode),
    Mouse(MouseButton),
}

impl Binding {
    /// Parses `--coop-ptt-key` / `--coop-ping-key` text: a `KeyCode` name (`Backslash`, `KeyB`, `F10`) or
    /// `MouseMiddle`, `MouseBack`, `MouseForward`, `MouseRight`.
    pub fn parse(s: &str) -> Option<Binding> {
        let k = |c: KeyCode| Some(Binding::Key(c));
        match s {
            "MouseMiddle" => Some(Binding::Mouse(MouseButton::Middle)),
            "MouseBack" => Some(Binding::Mouse(MouseButton::Back)),
            "MouseForward" => Some(Binding::Mouse(MouseButton::Forward)),
            "MouseRight" => Some(Binding::Mouse(MouseButton::Right)),
            "Backslash" => k(KeyCode::Backslash),
            "Backquote" => k(KeyCode::Backquote),
            "Insert" => k(KeyCode::Insert),
            "Delete" => k(KeyCode::Delete),
            "End" => k(KeyCode::End),
            "Home" => k(KeyCode::Home),
            "PageUp" => k(KeyCode::PageUp),
            "PageDown" => k(KeyCode::PageDown),
            "ControlRight" => k(KeyCode::ControlRight),
            "AltRight" => k(KeyCode::AltRight),
            "Semicolon" => k(KeyCode::Semicolon),
            "Quote" => k(KeyCode::Quote),
            "Period" => k(KeyCode::Period),
            "Comma" => k(KeyCode::Comma),
            "Slash" => k(KeyCode::Slash),
            "F7" => k(KeyCode::F7),
            "F8" => k(KeyCode::F8),
            "F10" => k(KeyCode::F10),
            "F11" => k(KeyCode::F11),
            other => {
                let letters = [
                    KeyCode::KeyA,
                    KeyCode::KeyB,
                    KeyCode::KeyC,
                    KeyCode::KeyD,
                    KeyCode::KeyE,
                    KeyCode::KeyF,
                    KeyCode::KeyG,
                    KeyCode::KeyH,
                    KeyCode::KeyI,
                    KeyCode::KeyJ,
                    KeyCode::KeyK,
                    KeyCode::KeyL,
                    KeyCode::KeyM,
                    KeyCode::KeyN,
                    KeyCode::KeyO,
                    KeyCode::KeyP,
                    KeyCode::KeyQ,
                    KeyCode::KeyR,
                    KeyCode::KeyS,
                    KeyCode::KeyT,
                    KeyCode::KeyU,
                    KeyCode::KeyV,
                    KeyCode::KeyW,
                    KeyCode::KeyX,
                    KeyCode::KeyY,
                    KeyCode::KeyZ,
                ];
                let letter = other
                    .strip_prefix("Key")
                    .filter(|l| l.len() == 1)
                    .and_then(|l| l.bytes().next());
                letter
                    .filter(u8::is_ascii_uppercase)
                    .and_then(|b| letters.get((b - b'A') as usize))
                    .map(|c| Binding::Key(*c))
            }
        }
    }

    /// Held now.
    // not called yet: the f71db09 feature merge left push to talk unwired
    #[allow(dead_code)]
    pub fn pressed(&self, keys: &ButtonInput<KeyCode>, mouse: &ButtonInput<MouseButton>) -> bool {
        match self {
            Binding::Key(k) => keys.pressed(*k),
            Binding::Mouse(m) => mouse.pressed(*m),
        }
    }

    /// Went down this frame.
    pub fn just_pressed(
        &self,
        keys: &ButtonInput<KeyCode>,
        mouse: &ButtonInput<MouseButton>,
    ) -> bool {
        match self {
            Binding::Key(k) => keys.just_pressed(*k),
            Binding::Mouse(m) => mouse.just_pressed(*m),
        }
    }
}

/// Settings from the command line, set once by [`configure`].
// `ptt`, `voice` and `gamepad` are not read yet: the f71db09 feature merge left them unwired
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Settings {
    pub ping: Binding,
    pub ptt: Binding,
    /// `off`, `ptt` or `vad`.
    pub voice: String,
    /// Couch mode: where to put the window (x, y, width, height).
    pub window: Option<(i32, i32, u32, u32)>,
    /// Couch mode: the controller this seat uses.
    pub gamepad: Option<u8>,
}

static SETTINGS: Mutex<Option<Settings>> = Mutex::new(None);

/// Reads the `--coop-*` social options (called from `coop::connect`).
pub fn configure(a: &crate::args::CoopArgs) {
    let ping = a
        .ping_key
        .as_deref()
        .and_then(Binding::parse)
        .unwrap_or(Binding::Mouse(MouseButton::Middle));
    let ptt = a
        .ptt_key
        .as_deref()
        .and_then(Binding::parse)
        .unwrap_or(Binding::Key(KeyCode::Backslash));
    let voice = a.voice.clone().unwrap_or_else(|| "ptt".into());
    if let Ok(mut s) = SETTINGS.lock() {
        *s = Some(Settings {
            ping,
            ptt,
            voice,
            window: a.window,
            gamepad: a.gamepad,
        });
        if let Some(g) = a.gamepad {
            println!("Co-op couch: seat uses controller {g} (the launcher routes the input)");
        }
    }
}

/// The settings (defaults before [`configure`]).
pub fn settings() -> Settings {
    SETTINGS
        .lock()
        .ok()
        .and_then(|s| s.clone())
        .unwrap_or(Settings {
            ping: Binding::Mouse(MouseButton::Middle),
            ptt: Binding::Key(KeyCode::Backslash),
            voice: "ptt".into(),
            window: None,
            gamepad: None,
        })
}

/// The most health points seen per player: the avatar stream carries points, not a maximum, so the
/// bar is points over the best seen.
static MAX_SEEN: Mutex<Vec<(u8, u16)>> = Mutex::new(Vec::new());

fn health_frac(p: PlayerId, hp: u16) -> f32 {
    let Ok(mut m) = MAX_SEEN.lock() else {
        return 1.0;
    };
    let max = match m.iter_mut().find(|(k, _)| *k == p.0) {
        Some(e) => {
            e.1 = e.1.max(hp);
            e.1
        }
        None => {
            m.push((p.0, hp));
            hp
        }
    };
    if max == 0 {
        1.0
    } else {
        f32::from(hp) / f32::from(max)
    }
}

/// The local player's own pings and waypoint.
#[derive(Resource, Default)]
pub struct Book(pub c::PingBook);

/// Teammates and pins the maps draw, refreshed every frame.
static PARTY: Mutex<Vec<[f32; 3]>> = Mutex::new(Vec::new());

/// Positions of teammates and pings in the player's own place, for the world
/// map and the local map.
pub fn party_points() -> Vec<[f32; 3]> {
    PARTY.lock().map(|p| p.clone()).unwrap_or_default()
}

/// The pins the HUD draws: teammates and other players' markers in this place, and the player's own.
pub fn pins(now: &LiveNow, book: &Book, gs: &world::scripting::GameState) -> Vec<c::Pin> {
    let Some(view) = &now.view else {
        return Vec::new();
    };
    let here = gs.player_cell.map_or(0, |c| c.0);
    let outdoors = gs.player_world.is_some();
    let me = gs.player_position.unwrap_or([0.0; 3]);
    let near = |cell: u32, pos: [f32; 3]| {
        (cell != 0 && cell == here)
            || (outdoors && cell != 0 && (pos[0] - me[0]).hypot(pos[1] - me[1]) <= REACH)
    };
    let mates = view
        .avatars
        .iter()
        .filter(|a| near(a.cell, a.pos))
        .map(|a| {
            // outdoors, other squares count as this place
            (
                a.player,
                a.name.clone(),
                here,
                a.pos,
                health_frac(a.player, a.health),
                a.downed,
            )
        });
    let marks = view
        .markers
        .iter()
        .filter(|m| near(m.cell, m.pos))
        .map(|m| (m.owner, m.kind, here, m.pos, m.label.clone()));
    let own: Vec<c::Pin> = book
        .0
        .pins()
        .into_iter()
        .filter(|p| near(p.cell, p.pos))
        .map(|p| c::Pin { cell: here, ..p })
        .collect();
    c::collect_pins(mates, marks, &own)
}

/// Adds teammates and pings to the HUD's compass input (`hud::HudState::input`).
pub fn compass_extra(
    now: &LiveNow,
    book: &Book,
    gs: &world::scripting::GameState,
    heading_deg: f32,
    markers: &mut Vec<CompassMarker>,
    actors: &mut Vec<CompassActor>,
) {
    let _ = heading_deg;
    let me = gs.player_position.unwrap_or([0.0; 3]);
    let l = c::Listener {
        cell: gs.player_cell.map_or(0, |c| c.0),
        pos: me,
        heading: 0.0,
    };
    for p in pins(now, book, gs) {
        let (_, d) = c::bearing(&l, p.pos);
        if p.kind == c::KIND_TEAMMATE {
            if d <= c::TAG_RANGE {
                actors.push(CompassActor {
                    position: p.pos,
                    hostile: false,
                });
            }
        } else {
            markers.push(CompassMarker {
                position: p.pos,
                found: true,
            });
        }
    }
}

/// Every frame: refreshes what the maps draw and expires the player's own pings.
pub fn publish(now: Res<LiveNow>, state: Res<DialogueState>, mut book: ResMut<Book>) {
    let Some(view) = &now.view else { return };
    book.0.tick(view.now_ms);
    let pts: Vec<[f32; 3]> = pins(&now, &book, &state.0)
        .into_iter()
        .map(|p| p.pos)
        .collect();
    if let Ok(mut g) = PARTY.lock() {
        *g = pts;
    }
}

/// The ping key.
#[allow(clippy::too_many_arguments)]
pub fn ping_input(
    keys: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut now: ResMut<LiveNow>,
    state: Res<DialogueState>,
    menus: Res<crate::menus::Menus>,
    conversation: Res<crate::dialogue::Conversation>,
    crosshair: Res<crate::crosshair::Crosshair>,
    cameras: Query<&Transform, With<crate::FlyCamera>>,
    mut book: ResMut<Book>,
    mut link: NonSendMut<CoopLink>,
    mut messages: ResMut<HudMessages>,
) {
    let Some((me, now_ms)) = now.view.as_ref().map(|v| (v.me, v.now_ms)) else {
        return;
    };
    if menus.is_open() || conversation.0.is_some() {
        return;
    }
    let key = settings().ping;
    if !key.just_pressed(&keys, &mouse) {
        return;
    }
    let Ok(t) = cameras.single() else { return };
    let gs = &state.0;
    let cell = gs.player_cell.map_or(0, |c| c.0);
    let f = t.forward().as_vec3();
    let eye = crate::walk::game_point(t.translation);
    let dir = [f.x, -f.z, f.y];
    let at = c::ping_point(eye, dir, crosshair.0.map(|p| p.distance));
    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    if ctrl {
        if let Some(m) = book.0.clear_waypoint() {
            link.0.live_intent(Intent::Social(m));
            messages
                .queue
                .push(HudMessage::from("Waypoint cleared".to_string()));
        }
    } else if shift {
        let m = book.0.waypoint(me, cell, at, "waypoint");
        link.0.live_intent(Intent::Social(m));
        now.counters.pings += 1;
        messages
            .queue
            .push(HudMessage::from("Waypoint set".to_string()));
    } else if let Some(msgs) = book.0.ping(me, cell, at, "ping", now_ms) {
        for m in msgs {
            link.0.live_intent(Intent::Social(m));
        }
        now.counters.pings += 1;
    }
}

/// The label of a name tag (name, distance), its colour, and the health
/// fraction for the bar when the player is hurt or near.
pub fn tag_text(
    now: &LiveNow,
    gs: &world::scripting::GameState,
    a: &coop_session::RemoteAvatarView,
    speaking: bool,
) -> (String, Color, Option<f32>) {
    let me = gs.player_position.unwrap_or([0.0; 3]);
    let l = c::Listener {
        cell: a.cell,
        pos: me,
        heading: 0.0,
    };
    let pin = c::Pin {
        owner: a.player,
        kind: c::KIND_TEAMMATE,
        cell: a.cell,
        pos: a.pos,
        label: a.name.clone(),
        health: Some(health_frac(a.player, a.health)),
        downed: a.downed,
    };
    let _ = now;
    let metres = ((a.pos[0] - me[0]).hypot(a.pos[1] - me[1]) / 70.0).round();
    let mut text = format!("{}{} {metres} m", if speaking { "> " } else { "" }, a.name);
    let mut tone = 0;
    let mut bar = None;
    if let Some(t) = c::name_tag(&l, &pin, speaking) {
        tone = t.tone;
        if t.show_bar {
            bar = Some(t.health);
        }
        if a.downed {
            text.push_str("\ndown");
        }
    }
    let color = if speaking {
        Color::srgb(0.45, 1.0, 0.45)
    } else {
        match tone {
            1 => Color::srgb(1.0, 0.85, 0.4),
            2 => Color::srgb(1.0, 0.45, 0.35),
            3 => Color::srgb(0.7, 0.7, 0.7),
            _ => Color::srgb(0.9, 0.95, 0.7),
        }
    };
    (text, color, bar)
}

/// Couch mode: puts the window where `--coop-window` says, once.
pub fn place_window(
    mut done: Local<bool>,
    mut windows: Query<&mut Window, With<bevy::window::PrimaryWindow>>,
) {
    if *done {
        return;
    }
    let Some((x, y, w, h)) = settings().window else {
        *done = true;
        return;
    };
    for mut win in &mut windows {
        win.decorations = false;
        win.position = bevy::window::WindowPosition::At(IVec2::new(x, y));
        win.resolution.set_physical_resolution(w, h);
        *done = true;
    }
}

/// The top-of-screen hint for pins outside the compass.
#[derive(Component)]
pub struct EdgeHint;

/// Spawns the hint text.
pub fn setup(mut commands: Commands) {
    commands.spawn((
        Text::new(String::new()),
        TextFont {
            font_size: 14.0,
            ..default()
        },
        TextColor(Color::srgb(0.85, 0.95, 1.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(6.0),
            left: Val::Percent(30.0),
            ..default()
        },
        Visibility::Hidden,
        EdgeHint,
    ));
}

/// Pins that are behind the player or off the strip, listed with their side.
pub fn edge_hints(
    now: Res<LiveNow>,
    state: Res<DialogueState>,
    book: Res<Book>,
    cameras: Query<&Transform, With<crate::FlyCamera>>,
    mut hint: Query<(&mut Text, &mut Visibility), With<EdgeHint>>,
) {
    let Ok(cam) = cameras.single() else { return };
    let f = cam.forward().as_vec3();
    let gs = &state.0;
    let l = c::Listener {
        cell: gs.player_cell.map_or(0, |c| c.0),
        pos: gs.player_position.unwrap_or([0.0; 3]),
        heading: f.x.atan2(-f.z),
    };
    let mut left = Vec::new();
    let mut right = Vec::new();
    for e in c::compass(&l, &pins(&now, &book, gs)) {
        let name = if e.label.is_empty() {
            "ping".to_string()
        } else {
            e.label.clone()
        };
        let item = format!("{name} {} m", (e.distance / 70.0).round());
        match e.edge {
            c::Edge::Left => left.push(item),
            c::Edge::Right => right.push(item),
            c::Edge::None => {}
        }
    }
    let text = match (left.is_empty(), right.is_empty()) {
        (true, true) => String::new(),
        (false, true) => format!("< {}", left.join(", ")),
        (true, false) => format!("{} >", right.join(", ")),
        _ => format!("< {}   |   {} >", left.join(", "), right.join(", ")),
    };
    for (mut t, mut v) in &mut hint {
        if t.0 != text {
            t.0 = text.clone();
        }
        *v = if text.is_empty() {
            Visibility::Hidden
        } else {
            Visibility::Inherited
        };
    }
}

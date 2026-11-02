//! The lockpicking menu on screen (`world::lockpick` the rules,
//! `ui::lockpick` the menu file's tiles, `cellview::lockpick` the 3D scene;
//! read from the game's code, `%USERPROFILE%\nv-re\findings\lockpick.md`).
//!
//! E on a locked door or container the player can pick (`world::locks::
//! try_open` says `Pick`) asks for the menu ([`Lockpicking::request`]); it
//! opens the next frame as the game's `0078db00` sets it up: the menu file
//! `menus\lockpick_menu.xml` laid out and filled (skill, bobby pins, the
//! lock's level, "Force Lock [n%]", the sweet spot's rings under the
//! hidden debug meter), and the lock and bobby pin models in a scene of
//! their own with the lock model's two point lights, seen by the menu's
//! camera (`0078e900`).
//!
//! Drawn as the game draws it (`00872940`): the 3D scene after the image
//! space pass (so neither graded nor bloomed) and the menu's pictures over
//! it, here both into the HUD's picture (`hud::HudLayer`), which the image
//! space pass lays over the finished scene: the scene by a camera of its
//! own, then the menu's quads by a 2D camera, each blended onto what's
//! there (premultiplied: the same as drawing them straight onto the
//! scene). The HUD itself is hidden while the menu is open (the viewer's
//! rule for every menu).
//!
//! Each frame (`0078eb50`): the game's whole milliseconds; the mouse's
//! movement across × the menu's width over the screen's pixel width
//! (`007908f0`); the four movement controls turn the cylinder (`00791540`:
//! Slide Left, Slide Right, Forward, Back, read from the INI's
//! `[Controls]`); F and E click the buttons the menu file names
//! (`_PCButton_F` Force Lock, `_PCButton_E` Exit). What the menu does is
//! carried out here: sounds by their editor IDs, the straining sound
//! faded, messages, a pin used up, the lock broken or picked; after a pick
//! the player uses the door or container (`00573170`), here as if E were
//! pressed on it again (`walk::doors`).
//!
//! Left out: the mouse cursor (hidden over the middle of the screen,
//! `007902b0`, but drawn over the buttons) and clicking the buttons with
//! it; the cursor's acceleration (`007118d0`, not traced); the tutorial
//! message the menu waits on (`+0x99`); controller rumble; the debug mode.

use std::collections::HashMap;
use std::sync::Arc;

use bevy::audio::{AudioSinkPlayback, Volume};
use bevy::core_pipeline::tonemapping::{DebandDither, Tonemapping};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::AccumulatedMouseMotion;
use bevy::prelude::*;
use bevy::render::camera::{CameraOutputMode, Exposure, RenderTarget};
use bevy::render::render_resource::{BlendState, WgpuFeatures};
use bevy::render::renderer::RenderDevice;
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;
use cellview::lockpick::{LockpickScene, MenuCamera};
use cellview::space;
use esm::FormId;
use ui::draw::{DrawItem, DrawKind};
use ui::lockpick::{LockpickMenu, RingTile, Texts};
use world::lockpick::{Effect, Frame, Picking, Refusal, Settings, Shows};
use world::scripting::{Event, GameState};

use crate::dialogue::DialogueState;
use crate::hud::{HudLayer, Quad, TileMaterial, TileParams};
use crate::lighting::{GameLighting, GameLitMaterial, MAX_LIGHTS};
use crate::sounds::PcmSound;
use crate::GameFiles;

/// The render layers the menu's 3D scene and its pictures are drawn on
/// (the HUD's is 23).
const LOCK_LAYER: usize = 25;
const MENU_LAYER: usize = 24;
/// Nothing is drawn here: the camera that clears the picture when the HUD
/// is off (`--no-hud`) sees it.
const CLEAR_LAYER: usize = 26;

/// The straining sound (`0078eb50`), kept to fade it.
const TENSION_SOUND: &str = "UILockpickingPickTensionLPM";
/// The lockpicking tutorial (`00790290`: 0x14, 0x1C with a pad, which
/// this menu doesn't take).
const TUTORIAL: u8 = world::tutorial::id::LOCKPICKING_PC;
/// Refusals' sound when the skill is too low (`0078db00`).
pub const POPUP_SOUND: &str = "UIPopUpMessageGeneral";

impl Lockpicking {
    /// Whether the menu is open.
    pub fn is_open(&self) -> bool {
        self.open.is_some()
    }
}

/// `--lockpick REF`: a lock to try once the place is up, as E on it.
#[derive(Resource, Default)]
pub struct StartLock(pub Option<String>);

/// The lockpicking menu: asked for, open, and what it put on screen.
#[derive(Resource, Default)]
pub struct Lockpicking {
    /// A lock the player tried and may pick: the menu opens on it.
    pub request: Option<FormId>,
    /// A lock just picked: the player uses it (`walk::doors`).
    pub again: Option<FormId>,
    open: Option<Box<Open>>,
    /// The menu's models, read the first time (`None` inside: they can't
    /// be read).
    models: Option<Option<Arc<LockpickScene>>>,
    shown: Option<Shown>,
    tension: Tension,
    /// The picture made here when the HUD is off.
    layer: Option<Handle<Image>>,
}

/// The open menu.
struct Open {
    picking: Picking,
    ui: ui::Ui,
    menu: LockpickMenu,
    models: Option<Arc<LockpickScene>>,
    /// The screen it was laid out for (physical pixels).
    size: UVec2,
    /// The game's millisecond clock at the last update.
    clock: u64,
}

/// The straining sound: starting, fading, playing.
#[derive(Default)]
struct Tension {
    start: bool,
    fade: Option<u32>,
    playing: Option<Entity>,
    /// Fading out: the sound, when its fade began and how long it takes
    /// (seconds).
    fading: Vec<(Entity, f32, f32)>,
}

/// What the open menu put on screen.
struct Shown {
    entities: Vec<Entity>,
    meshes: Vec<Handle<Mesh>>,
    materials: Vec<Handle<GameLitMaterial>>,
    images: Vec<Handle<Image>>,
    /// A white pixel: the alpha map of every piece (none has one here).
    white: Handle<Image>,
    pictures: MenuPictures,
}

/// One of the models' pieces: its model (`cellview::lockpick::
/// LOCK_REFERENCE` or `PIN_REFERENCE`), its mesh in the scene, and the point
/// it's built around.
#[derive(Component)]
struct LockPiece {
    reference: u32,
    mesh: usize,
    center: Vec3,
}

pub struct LockpickPlugin;

impl Plugin for LockpickPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lockpicking>().add_systems(
            Update,
            show_lockpicking
                // Kept: drawn as the menu ran this frame, inside the
                // interface idle's set (`crate::frame_order::IdleSet`).
                .after(pick_locks)
                .in_set(crate::frame_order::IdleSet),
        );
    }
}

/// A DirectInput key number (the INI's `[Controls]` values) as Bevy's key.
pub(crate) fn scan_code_key(code: u32) -> Option<KeyCode> {
    use KeyCode::*;
    Some(match code {
        0x01 => Escape,
        0x02 => Digit1,
        0x03 => Digit2,
        0x04 => Digit3,
        0x05 => Digit4,
        0x06 => Digit5,
        0x07 => Digit6,
        0x08 => Digit7,
        0x09 => Digit8,
        0x0A => Digit9,
        0x0B => Digit0,
        0x0C => Minus,
        0x0D => Equal,
        0x0E => Backspace,
        0x0F => Tab,
        0x10 => KeyQ,
        0x11 => KeyW,
        0x12 => KeyE,
        0x13 => KeyR,
        0x14 => KeyT,
        0x15 => KeyY,
        0x16 => KeyU,
        0x17 => KeyI,
        0x18 => KeyO,
        0x19 => KeyP,
        0x1A => BracketLeft,
        0x1B => BracketRight,
        0x1C => Enter,
        0x1D => ControlLeft,
        0x1E => KeyA,
        0x1F => KeyS,
        0x20 => KeyD,
        0x21 => KeyF,
        0x22 => KeyG,
        0x23 => KeyH,
        0x24 => KeyJ,
        0x25 => KeyK,
        0x26 => KeyL,
        0x27 => Semicolon,
        0x28 => Quote,
        0x29 => Backquote,
        0x2A => ShiftLeft,
        0x2B => Backslash,
        0x2C => KeyZ,
        0x2D => KeyX,
        0x2E => KeyC,
        0x2F => KeyV,
        0x30 => KeyB,
        0x31 => KeyN,
        0x32 => KeyM,
        0x33 => Comma,
        0x34 => Period,
        0x35 => Slash,
        0x36 => ShiftRight,
        0x37 => NumpadMultiply,
        0x38 => AltLeft,
        0x39 => Space,
        0x3A => CapsLock,
        0x3B => F1,
        0x3C => F2,
        0x3D => F3,
        0x3E => F4,
        0x3F => F5,
        0x40 => F6,
        0x41 => F7,
        0x42 => F8,
        0x43 => F9,
        0x44 => F10,
        0x47 => Numpad7,
        0x48 => Numpad8,
        0x49 => Numpad9,
        0x4A => NumpadSubtract,
        0x4B => Numpad4,
        0x4C => Numpad5,
        0x4D => Numpad6,
        0x4E => NumpadAdd,
        0x4F => Numpad1,
        0x50 => Numpad2,
        0x51 => Numpad3,
        0x52 => Numpad0,
        0x53 => NumpadDecimal,
        0x57 => F11,
        0x58 => F12,
        0x9C => NumpadEnter,
        0x9D => ControlRight,
        0xB5 => NumpadDivide,
        0xB8 => AltRight,
        0xC7 => Home,
        0xC8 => ArrowUp,
        0xC9 => PageUp,
        0xCB => ArrowLeft,
        0xCD => ArrowRight,
        0xCF => End,
        0xD0 => ArrowDown,
        0xD1 => PageDown,
        0xD2 => Insert,
        0xD3 => Delete,
        _ => return None,
    })
}

/// The keyboard key of a control in the INI's `[Controls]` (eight hex
/// digits; the keyboard's key number is the second byte from the left:
/// `Forward=0011FF13` is W, `Activate=0012FF0A` E), else `default`.
fn control_key(game: &cellview::Game, name: &str, default: u32) -> Option<KeyCode> {
    let code = game
        .settings
        .get("Controls", name)
        .and_then(|v| u32::from_str_radix(v.trim(), 16).ok())
        .map_or(default, |v| (v >> 16) & 0xFF);
    scan_code_key(code)
}

/// The four controls that turn the cylinder (`00791540`: Slide Left, Slide
/// Right, Forward, Back). The defaults are this install's values (the
/// exe's own defaults aren't traced).
fn turn_keys(game: &cellview::Game) -> Vec<KeyCode> {
    [
        ("Slide Left", 0x1E),
        ("Slide Right", 0x20),
        ("Forward", 0x11),
        ("Back", 0x1F),
    ]
    .into_iter()
    .filter_map(|(name, default)| control_key(game, name, default))
    .collect()
}

/// A sound record by editor ID, played as the game's code plays it.
fn sound(order: &esm::LoadOrder, state: &mut GameState, name: &str) {
    match order.form_by_editor_id(name) {
        Some(id) => state.events.push(Event::Sound(id)),
        None => println!("  (the sound {name} isn't in the data)"),
    }
}

/// A message in the HUD's corner, with its picture (`None` the neutral
/// Vault Boy).
fn message(state: &mut GameState, text: String, icon: Option<&str>) {
    state.events.push(Event::Message {
        title: None,
        text,
        buttons: Vec::new(),
        icon: icon.map(str::to_string),
    });
}

/// The text settings the menu file names (`&-sName;`): the data's, with
/// the exe's defaults for those it doesn't have.
fn menu_text_settings(order: &esm::LoadOrder) -> HashMap<String, String> {
    let mut settings = crate::hud::text_settings(order);
    for (name, text) in world::lockpick::EXE_TEXT {
        if !settings.keys().any(|k| k.eq_ignore_ascii_case(name)) {
            settings.insert(name.to_string(), text.to_string());
        }
    }
    settings
}

/// Opens the menu on `reference` (`0078db00`), or says why not.
fn open_menu(
    game: &cellview::Game,
    state: &mut GameState,
    reference: FormId,
    size: UVec2,
    models: Option<Arc<LockpickScene>>,
    now_ms: u64,
) -> Result<Box<Open>, String> {
    let order = &game.order;
    let lock = world::locks::lock_now(order, state, reference)
        .ok_or_else(|| format!("{reference} isn't locked"))?;
    let (difficulty, _) = lock.difficulty(order);
    let skill = world::lockpick::player_skill(order, state);
    let pins = world::lockpick::pins(order, state);
    let settings = Settings::load(order);

    let mut read = |p: &str| game.assets.read(p).ok().flatten();
    let ini = |s: &str, k: &str| game.settings.get(s, k).map(str::to_string);
    let mut ui = ui::game::new_ui(&mut read, &ini, menu_text_settings(order), size.x, size.y);
    let menu = LockpickMenu::load(&mut ui, &mut read, false)?;
    for w in &ui.warnings {
        println!("  lockpicking menu: {w}");
    }
    let meter = menu.meter_width(&mut ui);
    let spans = models.as_ref().map_or_else(Default::default, |m| {
        cellview::lockpick::spans(&m.lock_sequences, &m.pin_sequences)
    });
    // The sweet spot's place: a random share of the meter.
    let random01 = (state.roll() >> 40) as f32 / (1u64 << 24) as f32;
    let picking = match Picking::open(
        reference, difficulty, skill, pins, settings, meter, random01, spans,
    ) {
        Ok(p) => p,
        Err(Refusal::SkillTooLow(needs)) => {
            sound(order, state, POPUP_SOUND);
            return Err(world::lockpick::message_text(
                order,
                "sLockpickSkillTooLow",
                "You need a lockpick skill of %d to pick this lock.",
            )
            .replace("%d", &needs.to_string()));
        }
    };
    let text = |name: &str| world::lockpick::text(order, name).unwrap_or_default();
    // In English the code writes the titles itself (the same settings the
    // file names).
    let english = game
        .settings
        .get("General", "sLanguage")
        .is_some_and(|l| l.trim().eq_ignore_ascii_case("ENGLISH"))
        .then(|| {
            [
                text("sLockpickSkillText"),
                text("sPicksRemainingText"),
                text("sLockLevelText"),
                text("sExit"),
            ]
        });
    menu.fill(
        &mut ui,
        &Texts {
            skill,
            force_lock: picking.force_label(&text("sForceLock")),
            level: world::lockpick::level_name(order, difficulty),
            pins,
            english,
        },
    );
    let rings: Vec<RingTile> = picking
        .zone
        .rings
        .iter()
        .map(|r| RingTile {
            id: r.id,
            x: r.x,
            width: r.width,
            depth: r.depth,
            color: r.color,
        })
        .collect();
    menu.place_rings(&mut ui, &rings);
    println!(
        "Picking a lock ({}; Lockpick {skill}, {pins} bobby pins).",
        world::lockpick::level_name(order, difficulty)
    );
    Ok(Box::new(Open {
        picking,
        ui,
        menu,
        models,
        size,
        clock: now_ms,
    }))
}

/// The keyboard and mouse, which the menu takes while it's open.
#[derive(bevy::ecs::system::SystemParam)]
pub struct MenuInput<'w, 's> {
    keys: ResMut<'w, ButtonInput<KeyCode>>,
    buttons: ResMut<'w, ButtonInput<MouseButton>>,
    motion: ResMut<'w, AccumulatedMouseMotion>,
    typed: EventReader<'w, 's, KeyboardInput>,
}

/// Opens the menu when asked, and runs it each frame: the keyboard and
/// mouse are the menu's while it's open.
#[allow(clippy::too_many_arguments)]
pub fn pick_locks(
    time: Res<Time>,
    game: Res<GameFiles>,
    mut state: ResMut<DialogueState>,
    mut lockpicking: ResMut<Lockpicking>,
    mut menus: ResMut<crate::menus::Menus>,
    mut player: ResMut<crate::walk::Player>,
    input: MenuInput,
    windows: Query<&Window, With<PrimaryWindow>>,
    (mut start, commands): (ResMut<StartLock>, Res<crate::scripts::StartCommands>),
) {
    let MenuInput {
        mut keys,
        mut buttons,
        mut motion,
        mut typed,
    } = input;
    let order = &game.0.order;
    let state = &mut state.0;
    let now_ms = time.elapsed().as_millis() as u64;
    let lockpicking = &mut *lockpicking;
    // `--lockpick`: the lock tried once the place is up and `--run`'s lines
    // have run.
    if player.ready && commands.0.is_empty() {
        if let Some(name) = start.0.take() {
            let found = order.form_by_editor_id(&name).or_else(|| {
                u32::from_str_radix(name.trim_start_matches("0x"), 16)
                    .ok()
                    .map(FormId)
            });
            match found {
                Some(r) => match crate::scripts::locked(order, state, r, &name) {
                    None => {
                        println!("--lockpick: {name} isn't locked (or the player has its key).")
                    }
                    Some(crate::scripts::Locked::Pick) => lockpicking.request = Some(r),
                    Some(crate::scripts::Locked::Says(why)) => {
                        println!("{why}");
                        message(state, why, Some(world::message_icon::PADLOCK));
                    }
                },
                None => println!("--lockpick: there's no {name}."),
            }
        }
    }
    if lockpicking.open.is_none() {
        let Some(reference) = lockpicking.request.take() else {
            return;
        };
        if menus.is_open() {
            return;
        }
        let size = windows
            .single()
            .map(|w| UVec2::new(w.physical_width(), w.physical_height()))
            .unwrap_or(UVec2::new(1920, 1080))
            .max(UVec2::ONE);
        let models = lockpicking
            .models
            .get_or_insert_with(|| {
                let m = game.0.lockpick_scene().map(Arc::new);
                if m.is_none() {
                    println!("The lockpicking menu's models can't be read.");
                }
                m
            })
            .clone();
        match open_menu(&game.0, state, reference, size, models, now_ms) {
            Ok(open) => {
                lockpicking.open = Some(open);
                menus.lockpicking = true;
                player.ready = false;
            }
            Err(why) => {
                println!("{why}");
                // The skill refusal's padlock (`0078db00`).
                message(state, why, Some(world::lockpick::PADLOCK_ICON));
                return;
            }
        }
    }
    let Some(open) = lockpicking.open.as_mut() else {
        return;
    };
    // A game menu over it (its tutorial) hands the player back when it
    // closes; the lockpicking menu keeps them.
    player.ready = false;

    // The game's frame time in whole milliseconds (a millisecond clock's
    // steps, so nothing is lost between frames).
    let ms = now_ms.saturating_sub(open.clock) as u32;
    open.clock = now_ms;
    let turn = turn_keys(&game.0);
    let frame = Frame {
        ms,
        // On top unless one of the game's menus (its tutorial) is over it.
        active: !menus.game_open,
        mouse: motion.delta.x * open.ui.screen_size.resolution_converter(),
        turn_pressed: turn.iter().any(|&k| keys.just_pressed(k)),
        turn_held: turn.iter().any(|&k| keys.pressed(k)),
        tutorial_unseen: !state.tutorials.is_shown(TUTORIAL),
    };
    let mut effects = Vec::new();
    // F and E click the buttons the menu names (`HandleClick`, `00790330`).
    for (key, letter) in [(KeyCode::KeyF, 'F'), (KeyCode::KeyE, 'E')] {
        if !keys.just_pressed(key) {
            continue;
        }
        let clicked = open
            .menu
            .pc_button(&mut open.ui, letter)
            .and_then(|tile| open.menu.id_of(tile));
        match clicked {
            Some(ui::lockpick::FORCE_LOCK) => {
                let roll = (state.roll() % 100) as u32;
                effects.extend(open.picking.force(roll));
            }
            Some(ui::lockpick::EXIT) => effects.extend(open.picking.exit()),
            _ => {}
        }
    }
    if !effects.contains(&Effect::Close) {
        effects.extend(
            open.picking
                .update(&frame, &mut || (state.roll() % 10) as u32),
        );
    }
    open.menu.set_pick_x(&mut open.ui, open.picking.pick_x);

    let reference = open.picking.reference;
    let difficulty = open.picking.difficulty;
    let mut close = false;
    for effect in effects {
        match effect {
            Effect::Sound(name) => sound(order, state, name),
            Effect::TensionStart => lockpicking.tension.start = true,
            Effect::TensionFade(ms) => lockpicking.tension.fade = Some(ms),
            Effect::Message {
                setting,
                default,
                icon,
                sound: s,
            } => {
                message(
                    state,
                    world::lockpick::message_text(order, setting, default),
                    Some(icon),
                );
                if let Some(s) = s {
                    sound(order, state, s);
                }
            }
            Effect::Opened => {
                world::lockpick::picked(order, state, reference, difficulty);
                println!("Picked the lock.");
                lockpicking.again = Some(reference);
            }
            Effect::PinBroken => {
                world::lockpick::use_up_pin(order, state);
                if let Some(open) = lockpicking.open.as_mut() {
                    open.menu.set_pins(&mut open.ui, open.picking.pins);
                }
            }
            Effect::LockBroken => {
                world::lockpick::break_lock(state, reference);
                println!("The lock is broken: only its key opens it now.");
            }
            Effect::Close => close = true,
            // `ShowMessage(id, 1014, 0)`: the menu waits if it was asked
            // for (`+0x99`).
            Effect::Tutorial => {
                let wait = world::tutorial::ask(
                    order,
                    &mut state.tutorials,
                    TUTORIAL,
                    world::tutorial::menu::LOCKPICK,
                    0,
                );
                if let Some(open) = lockpicking.open.as_mut() {
                    open.picking.tutorial_wait = wait;
                }
            }
        }
    }
    if close {
        lockpicking.open = None;
        menus.lockpicking = false;
        player.ready = !menus.is_open();
    }
    // The menu has the keyboard and mouse.
    typed.clear();
    keys.reset_all();
    buttons.reset_all();
    motion.delta = Vec2::ZERO;
}

/// Pictures uploaded for the menu's pieces, and its pieces on screen
/// (as `hud` puts the HUD's).
#[derive(Default)]
struct MenuPictures {
    sizes: HashMap<String, Option<(u32, u32)>>,
    atlases: HashMap<String, Option<ui::Atlas>>,
    images: HashMap<(String, bool, bool), Option<Handle<Image>>>,
    font_images: HashMap<(usize, u32), Option<Handle<Image>>>,
    last: Vec<DrawItem>,
    drawn: Vec<(Entity, Handle<Mesh>, Handle<TileMaterial>)>,
}

/// Where the menu's pictures go.
#[derive(bevy::ecs::system::SystemParam)]
pub struct LockpickAssets<'w> {
    images: ResMut<'w, Assets<Image>>,
    meshes: ResMut<'w, Assets<Mesh>>,
    lit: ResMut<'w, Assets<GameLitMaterial>>,
    tiles: ResMut<'w, Assets<TileMaterial>>,
    wavs: ResMut<'w, Assets<PcmSound>>,
    device: Option<Res<'w, RenderDevice>>,
}

impl MenuPictures {
    /// Puts the menu's draw list on screen when it has changed: each piece
    /// a quad, its texture × its colour, blended in the list's order.
    #[allow(clippy::too_many_arguments)]
    fn paint(
        &mut self,
        commands: &mut Commands,
        game: &cellview::Game,
        ui: &mut ui::Ui,
        menu: ui::TileId,
        size: UVec2,
        white: &Handle<Image>,
        assets: &mut LockpickAssets,
        compressed: bool,
    ) {
        let items = {
            let mut files = crate::hud::Files {
                game,
                sizes: &mut self.sizes,
                atlases: &mut self.atlases,
            };
            ui::draw_list(ui, menu, &mut files, &|_| None)
        };
        if items == self.last {
            return;
        }
        self.clear(commands, assets);
        let k = 1.0 / ui.screen_size.resolution_converter();
        for (i, item) in items.iter().enumerate() {
            let tint = Vec4::from_array(item.color);
            let mut pieces: Vec<(Handle<Image>, Vec<Quad>)> = Vec::new();
            match &item.kind {
                DrawKind::Image {
                    texture,
                    rect,
                    uv,
                    repeat_u,
                    scroll: _,
                } => {
                    let repeat = (*repeat_u, false);
                    let handle = self
                        .images
                        .entry((texture.clone(), repeat.0, repeat.1))
                        .or_insert_with(|| {
                            crate::hud::upload_picture(
                                &mut assets.images,
                                game,
                                texture,
                                repeat,
                                compressed,
                            )
                        })
                        .clone();
                    let Some(handle) = handle else {
                        continue;
                    };
                    let corners = [
                        [uv[0], uv[1]],
                        [uv[2], uv[1]],
                        [uv[0], uv[3]],
                        [uv[2], uv[3]],
                    ];
                    pieces.push((handle, vec![(*rect, corners)]));
                }
                DrawKind::Model { .. } => continue,
                DrawKind::Text { font, glyphs } => {
                    let Some(f) = ui.fonts.get(font - 1).cloned().flatten() else {
                        continue;
                    };
                    let paths = ui::draw::font_textures(&f);
                    let mut by_picture: HashMap<u32, Vec<Quad>> = HashMap::new();
                    for (rect, uv, picture) in glyphs {
                        by_picture.entry(*picture).or_default().push((*rect, *uv));
                    }
                    let mut pictures: Vec<_> = by_picture.into_iter().collect();
                    pictures.sort_by_key(|(p, _)| *p);
                    for (picture, quads) in pictures {
                        let Some(path) = paths.get(picture as usize) else {
                            continue;
                        };
                        let handle = self
                            .font_images
                            .entry((*font, picture))
                            .or_insert_with(|| {
                                crate::hud::upload_font_picture(&mut assets.images, game, path)
                            })
                            .clone();
                        if let Some(texture) = handle {
                            pieces.push((texture, quads));
                        }
                    }
                }
            }
            for (texture, quads) in pieces {
                let mesh = assets.meshes.add(crate::hud::quads_mesh(&quads, k, size));
                let material = assets.tiles.add(TileMaterial {
                    params: TileParams {
                        tint,
                        scroll: Vec4::new(0.0, 0.0, 1.0, 1.0),
                        mode: Vec4::ZERO,
                    },
                    texture,
                    alpha_map: white.clone(),
                    blend: None,
                });
                let entity = commands
                    .spawn((
                        Mesh2d(mesh.clone()),
                        MeshMaterial2d(material.clone()),
                        Transform::from_xyz(0.0, 0.0, i as f32 * 0.01),
                        RenderLayers::layer(MENU_LAYER),
                    ))
                    .id();
                self.drawn.push((entity, mesh, material));
            }
        }
        self.last = items;
    }

    /// Takes the pieces off screen.
    fn clear(&mut self, commands: &mut Commands, assets: &mut LockpickAssets) {
        for (e, mesh, material) in self.drawn.drain(..) {
            commands.entity(e).despawn();
            assets.meshes.remove(&mesh);
            assets.tiles.remove(&material);
        }
        self.last.clear();
    }

    /// Lets go of every picture uploaded.
    fn forget(&mut self, assets: &mut LockpickAssets) {
        for h in self
            .images
            .drain()
            .filter_map(|(_, h)| h)
            .chain(self.font_images.drain().filter_map(|(_, h)| h))
        {
            assets.images.remove(&h);
        }
    }
}

/// The lighting of the menu's scene (`0078e1c0`): the lock model's point
/// lights only (`00b5c940`), the menus' scene's own light black, no fog.
fn scene_lighting(models: &LockpickScene, brightness: f32) -> GameLighting {
    menu_lighting(&models.lights, brightness)
}

/// Lighting shared by the game's independent 3D menu scenes.
pub(crate) fn menu_lighting(lights: &[cellview::LightData], brightness: f32) -> GameLighting {
    GameLighting {
        ambient: Vec4::ZERO,
        directional_color: Vec4::ZERO,
        directional_direction: Vec4::Y,
        emissive: Vec4::ZERO,
        scale: Vec4::new(
            crate::full_brightness_nits(),
            lights.len().min(MAX_LIGHTS) as f32,
            0.0,
            0.0,
        ),
        fog_color: Vec4::ZERO,
        fog_range: Vec4::ZERO,
        specular: Vec4::ZERO,
        surface: Vec4::ZERO,
        falloff: crate::NO_FALLOFF,
        environment: Vec4::ZERO,
        draw: Vec4::ZERO,
        lights: crate::game_lights(lights, brightness),
        actor: Vec4::ZERO,
        hair_tint: Vec4::ZERO,
    }
}

/// The picture the menu draws into: the HUD's, or (with the HUD off) one
/// made here with a camera that clears it each frame, as the HUD's does.
pub(crate) fn menu_layer(
    commands: &mut Commands,
    hud: Option<&HudLayer>,
    made: &mut Option<Handle<Image>>,
    images: &mut Assets<Image>,
    size: UVec2,
) -> Handle<Image> {
    if let Some(layer) = hud {
        return layer.0.clone();
    }
    if let Some(layer) = made {
        return layer.clone();
    }
    use bevy::asset::RenderAssetUsages;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
    let mut image = Image::new_uninit(
        Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        TextureFormat::Rgba16Float,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;
    let layer = images.add(image);
    commands.insert_resource(HudLayer(layer.clone()));
    commands.spawn((
        Camera2d,
        Camera {
            target: RenderTarget::from(layer.clone()),
            order: -4,
            hdr: true,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        Tonemapping::None,
        DebandDither::Disabled,
        Msaa::Off,
        RenderLayers::layer(CLEAR_LAYER),
    ));
    *made = Some(layer.clone());
    layer
}

/// Puts the open menu on screen: its scene and pictures, moved each frame
/// as the menu has them; the straining sound; everything taken away when
/// it closes.
#[allow(clippy::too_many_arguments)]
fn show_lockpicking(
    mut commands: Commands,
    time: Res<Time>,
    game: Res<GameFiles>,
    settings: Res<crate::Settings>,
    state: Res<DialogueState>,
    mut lockpicking: ResMut<Lockpicking>,
    hud_layer: Option<Res<HudLayer>>,
    mut assets: LockpickAssets,
    mut pieces: Query<(&LockPiece, &mut Transform)>,
    mut sinks: Query<&mut AudioSink>,
) {
    let Lockpicking {
        open,
        shown,
        tension,
        layer,
        ..
    } = &mut *lockpicking;
    let now = time.elapsed_secs();

    // The straining sound: started, faded linearly over the code's
    // milliseconds, then stopped (`00ad8da0`).
    if std::mem::take(&mut tension.start) {
        if let Some(e) = tension.playing.take() {
            tension.fading.push((e, now, 0.0));
        }
        if let Some(id) = game.0.order.form_by_editor_id(TENSION_SOUND) {
            if let Some(s) = world::sound::Sound::load(&game.0.order, id) {
                tension.playing = crate::sounds::play(
                    &mut commands,
                    &game.0,
                    &mut assets.wavs,
                    &s,
                    state.0.dice,
                    false,
                );
            }
        }
    }
    if let Some(ms) = tension.fade.take() {
        if let Some(e) = tension.playing.take() {
            tension.fading.push((e, now, ms as f32 / 1000.0));
        }
    }
    tension.fading.retain(|&(e, at, seconds)| {
        let left = if seconds > 0.0 {
            1.0 - (now - at) / seconds
        } else {
            0.0
        };
        if left <= 0.0 {
            if let Ok(mut entity) = commands.get_entity(e) {
                entity.despawn();
            }
            return false;
        }
        if let Ok(mut sink) = sinks.get_mut(e) {
            sink.set_volume(Volume::Linear(left));
        }
        true
    });

    let Some(open) = open.as_mut() else {
        // Closed: everything taken away.
        if let Some(mut s) = shown.take() {
            for e in s.entities {
                commands.entity(e).despawn();
            }
            for m in s.meshes {
                assets.meshes.remove(&m);
            }
            for m in s.materials {
                assets.lit.remove(&m);
            }
            s.pictures.clear(&mut commands, &mut assets);
            s.pictures.forget(&mut assets);
            for i in s.images {
                assets.images.remove(&i);
            }
        }
        return;
    };
    let compressed = assets
        .device
        .as_ref()
        .is_none_or(|d| d.features().contains(WgpuFeatures::TEXTURE_COMPRESSION_BC));

    if shown.is_none() {
        // Into the HUD's picture: before the HUD's camera, which then lays
        // the game's menus (the tutorial box) over it, blended
        // (`game_menus::compose_hud_over_scene`), the scene's camera writing the
        // picture afresh. Into a picture of its own (no HUD): after its
        // clearing camera.
        let (scene_order, pictures_order, scene_output) = if hud_layer.is_some() {
            (
                -6,
                -5,
                CameraOutputMode::Write {
                    blend_state: None,
                    clear_color: ClearColorConfig::Custom(Color::NONE),
                },
            )
        } else {
            (
                -3,
                -2,
                CameraOutputMode::Write {
                    blend_state: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    clear_color: ClearColorConfig::None,
                },
            )
        };
        let target = menu_layer(
            &mut commands,
            hud_layer.as_deref(),
            layer,
            &mut assets.images,
            open.size,
        );
        let white = white_pixel(&mut assets.images);
        let mut s = Shown {
            entities: Vec::new(),
            meshes: Vec::new(),
            materials: Vec::new(),
            images: vec![white.clone()],
            white,
            pictures: MenuPictures::default(),
        };
        // The menu's camera (`0078e900`): at the scene's origin looking
        // along +x with +y up, the game's frustum, after the HUD's camera
        // (which clears the picture) and before the menu's pictures.
        let camera = MenuCamera::new(&game.0.settings, open.size.x, open.size.y);
        let blend_over = CameraOutputMode::Write {
            blend_state: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
            clear_color: ClearColorConfig::None,
        };
        s.entities.push(
            commands
                .spawn((
                    Camera3d::default(),
                    Camera {
                        target: RenderTarget::from(target.clone()),
                        order: scene_order,
                        hdr: true,
                        clear_color: ClearColorConfig::Custom(Color::NONE),
                        output_mode: scene_output,
                        ..default()
                    },
                    Tonemapping::None,
                    DebandDither::Disabled,
                    // No light clusters: nothing here is lit by Bevy's lights.
                    bevy::pbr::ClusterConfig::None,
                    Projection::from(PerspectiveProjection {
                        fov: camera.vertical_fov(),
                        near: camera.near * space::METERS_PER_UNIT,
                        far: 100.0,
                        ..default()
                    }),
                    Exposure {
                        ev100: crate::START_EV100,
                    },
                    // Scene +x (forward) is Bevy's +x, scene +y (up) Bevy's
                    // −z (`space`'s conversion).
                    Transform::from_translation(Vec3::ZERO).looking_to(Vec3::X, Vec3::NEG_Z),
                    RenderLayers::layer(LOCK_LAYER),
                ))
                .id(),
        );
        s.entities.push(
            commands
                .spawn((
                    Camera2d,
                    Camera {
                        target: RenderTarget::from(target),
                        order: pictures_order,
                        hdr: true,
                        clear_color: ClearColorConfig::Custom(Color::NONE),
                        output_mode: blend_over,
                        ..default()
                    },
                    Tonemapping::None,
                    DebandDither::Disabled,
                    Msaa::Off,
                    RenderLayers::layer(MENU_LAYER),
                ))
                .id(),
        );
        // The models' pieces.
        if let Some(models) = &open.models {
            let scene = &models.scene;
            let textures: Vec<Option<Handle<Image>>> = scene
                .textures
                .iter()
                .map(|t| {
                    crate::upload_texture(&mut assets.images, t, compressed, settings.anisotropy)
                })
                .collect();
            s.images.extend(textures.iter().flatten().cloned());
            let lighting = scene_lighting(models, settings.brightness);
            for draw in &scene.draws {
                let data = &scene.meshes[draw.mesh];
                let center = crate::sort_center(data).unwrap_or([0.0; 3]);
                let mesh = assets.meshes.add(crate::game_mesh_around(data, center));
                let material = assets
                    .lit
                    .add(crate::lit_material(data, &textures, lighting));
                s.meshes.push(mesh.clone());
                s.materials.push(material.clone());
                s.entities.push(
                    commands
                        .spawn((
                            Mesh3d(mesh),
                            MeshMaterial3d(material),
                            Transform::IDENTITY,
                            RenderLayers::layer(LOCK_LAYER),
                            crate::shared_light::MenuLit,
                            LockPiece {
                                reference: draw.reference,
                                mesh: draw.mesh,
                                center: Vec3::from(center),
                            },
                        ))
                        .id(),
                );
            }
        }
        *shown = Some(s);
    }
    let Some(s) = shown.as_mut() else {
        return;
    };

    // The pieces where the menu has them: each model's sequence at its
    // time, the pin turned for the pick.
    if let Some(models) = &open.models {
        let scene = &open.picking.scene;
        let width = open.picking.zone.meter_width;
        let sequence_name = |shows: Shows| match shows {
            Shows::Forward => "Forward",
            Shows::Backward => "Backward",
            Shows::Left => "Left",
        };
        for (piece, mut transform) in &mut pieces {
            let (list, (shows, at)) = if piece.reference == cellview::lockpick::PIN_REFERENCE {
                (&models.pin_sequences, scene.pin)
            } else {
                (&models.lock_sequences, scene.lock)
            };
            let layer = cellview::lockpick::sequence(list, sequence_name(shows)).map(|s| (s, at));
            let motion = models.scene.meshes[piece.mesh].motion.as_deref();
            let m = models.piece_matrix(piece.reference, motion, layer, (scene.pin_x, width));
            let wanted = Transform::from_matrix(
                Mat4::from_cols_array(&space::matrix(&m)) * Mat4::from_translation(piece.center),
            );
            if *transform != wanted {
                *transform = wanted;
            }
        }
    }

    // The menu's pictures.
    let menu = open.menu.menu;
    s.pictures.paint(
        &mut commands,
        &game.0,
        &mut open.ui,
        menu,
        open.size,
        &s.white,
        &mut assets,
        compressed,
    );
}

/// A white pixel: the alpha map of pieces without one.
fn white_pixel(images: &mut Assets<Image>) -> Handle<Image> {
    use bevy::asset::RenderAssetUsages;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
    let mut white = Image::new_fill(
        Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[255, 255, 255, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::RENDER_WORLD,
    );
    white.sampler = crate::hud::sampler(false, false);
    images.add(white)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn controls_are_read_as_the_ini_writes_them() {
        // This install's `[Controls]`: Forward=0011FF13 (W), Back=001FFF14
        // (S), Slide Left=001EFF17 (A), Slide Right=0020FF16 (D),
        // Activate=0012FF0A (E).
        for (value, key) in [
            (0x0011_FF13u32, KeyCode::KeyW),
            (0x001F_FF14, KeyCode::KeyS),
            (0x001E_FF17, KeyCode::KeyA),
            (0x0020_FF16, KeyCode::KeyD),
            (0x0012_FF0A, KeyCode::KeyE),
        ] {
            assert_eq!(scan_code_key((value >> 16) & 0xFF), Some(key));
        }
        assert_eq!(scan_code_key(0xFF), None);
    }
}

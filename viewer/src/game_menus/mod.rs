//! The game's own menus on screen (`ui::menu`, `ui::menus`): laid out from
//! the game's menu files and filled the way its menu classes fill them,
//! drawn over the scene with the HUD (`hud`), and driven by the mouse and
//! keyboard as the game's interface manager hands them on (`0070c4a0` in
//! FalloutNV.exe): the tile under the pointer is "moused over" and a click
//! goes to its menu; typed keys, the arrows and Enter go to the top menu.
//!
//! Each menu has a module of its own here, which takes its requests from
//! the game (`menus::Menu`, queued by scripts and the world) and hands the
//! player's choices back to the world.
//!
//! The pointer is the system's mouse pointer (the game moves its own by the
//! mouse's raw movement, at its own speed: `007118d0`); the game's cursor
//! picture (`sCursorFilename`, 32 × 32, drawn above every menu, `0070b240`)
//! is drawn where it is.

pub mod asks;
pub mod barter;
pub mod blackjack;
pub mod caravan;
pub mod casino;
pub mod chargen;
pub mod companion_wheel;
pub mod computers;
pub mod container;
pub mod dialog;
pub mod hacking;
pub mod levelup;
pub(crate) mod message;
pub mod recipe;
pub mod repair;
pub mod roulette;
pub mod sleepwait;
pub mod slots;
pub mod start;
pub mod textedit;
pub mod traits;
pub mod tutorial;
pub mod vigor;

use std::collections::HashMap;

use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseScrollUnit};
use bevy::input::ButtonState;
use bevy::prelude::*;
use bevy::render::camera::CameraOutputMode;
use bevy::render::render_resource::BlendState;
use bevy::window::PrimaryWindow;
use cellview::{Game, TextureData};
use ui::draw::{DrawItem, DrawKind, Textures};
use ui::menu::{Effect, Interface, MenuCode};
use ui::names::{kind, t};
use ui::TileId;

use crate::GameFiles;

/// The game's menus, built for the window's size, and the ones open.
#[derive(Resource, Default)]
pub struct GameMenus {
    pub(crate) screen: Option<Box<Screen>>,
    failed: bool,
    /// `--open-menu levelup:perks`: the next level-up menu's points are
    /// given and Continue pressed through the menu's own clicks.
    levelup_to_perks: bool,
    /// `--open-menu wait:N` / `sleep:N`: the next sleep/wait menu chooses
    /// N hours and presses Wait through its own click.
    sleepwait_hours: Option<u32>,
}

/// What the menus draw this frame, for the HUD's layer (`hud`), over the
/// HUD; and the open menus' class numbers, bottom first (the HUD shows
/// some of its pieces by them).
#[derive(Resource, Default)]
pub struct MenuDraw(pub Vec<DrawItem>, pub Vec<i32>);

/// The menus' tiles (one tile tree, as the game's), the interface's
/// pointer state, and the open menus, bottom first.
pub struct Screen {
    pub ui: ui::Ui,
    pub interface: Interface,
    pub open: Vec<OpenMenu>,
    /// Menus closed and fading out (`ui::fade`): drawn, not run; bottom
    /// first.
    pub fading: Vec<OpenMenu>,
    /// The menus' fade states and the fades running (`ui::fade`).
    pub fades: ui::fade::Fades,
    size: UVec2,
    /// The cursor's picture.
    cursor: Option<TileId>,
    /// The fade to black (the fader manager's fader 0, `Interface\Faders\
    /// Black.dds` over the scene, under the menus: a sleep's), and its
    /// picture.
    pub fade: Option<world::living::sleep::Fade>,
    fader: Option<TileId>,
    /// The Rest control (T) held down (the sleep/wait menu cancels on it).
    rest_down: bool,
    /// The Pip-Boy's callback for the "how many?" open, if it asked
    /// (`asks`).
    quantity_owner: Option<u32>,
    /// The pointer this frame, in menu units.
    pub pointer: Option<(f32, f32)>,
    /// The mouse's movement this frame, in pixels (roulette's cursor).
    pub mouse_move: (f32, f32),
    /// The player left a terminal or the hacking game this frame (their
    /// `Leave`): the rendered terminal fades out (`rendered_terminal`).
    pub terminal_left: bool,
    sizes: HashMap<String, Option<(u32, u32)>>,
    atlases: HashMap<String, Option<ui::Atlas>>,
    /// `nif` tiles' models (the start menu's pause background).
    models: HashMap<String, Option<Vec<start::ModelPiece>>>,
}

/// How a closed menu leaves (`Screen::close`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Close {
    Fade,
    Instant,
    Now,
}

/// A menu on screen.
pub enum OpenMenu {
    Message(ui::menus::message::MessageMenu),
    Dialog(ui::menus::dialog::DialogMenu),
    Container(Box<container::ContainerScreen>),
    Barter(Box<barter::BarterScreen>),
    Recipe(Box<recipe::RecipeScreen>),
    Repair(Box<repair::RepairScreen>),
    CompanionWheel(Box<companion_wheel::WheelScreen>),
    Caravan(Box<caravan::CaravanScreen>),
    Slots(Box<slots::SlotsScreen>),
    Blackjack(Box<blackjack::BlackjackScreen>),
    Roulette(Box<roulette::RouletteScreen>),
    Quantity(ui::menus::quantity::QuantityMenu),
    LevelUp(Box<ui::menus::levelup::LevelUpMenu>),
    Traits(Box<ui::menus::traits::TraitMenu>),
    CharGen(Box<ui::menus::chargen::CharGenMenu>),
    TextEdit(Box<ui::menus::textedit::TextEditMenu>),
    SleepWait(Box<ui::menus::sleepwait::SleepWaitMenu>),
    Vigor(Box<vigor::VigorScreen>),
    Start(Box<start::StartScreen>),
    Hacking(Box<hacking::HackingScreen>),
    Computers(Box<computers::ComputersScreen>),
    Tutorial(Box<ui::menus::tutorial::TutorialMenu>),
}

impl OpenMenu {
    fn code(&mut self) -> &mut dyn MenuCode {
        match self {
            OpenMenu::Message(m) => m,
            OpenMenu::Dialog(m) => m,
            OpenMenu::Container(c) => &mut c.menu,
            OpenMenu::Barter(b) => &mut b.menu,
            OpenMenu::Recipe(r) => &mut r.menu,
            OpenMenu::Repair(r) => &mut r.menu,
            OpenMenu::CompanionWheel(w) => &mut w.menu,
            OpenMenu::Caravan(c) => &mut c.menu,
            OpenMenu::Slots(s) => &mut s.menu,
            OpenMenu::Blackjack(b) => &mut b.menu,
            OpenMenu::Roulette(r) => &mut r.menu,
            OpenMenu::Quantity(m) => m,
            OpenMenu::LevelUp(m) => &mut **m,
            OpenMenu::Traits(m) => &mut **m,
            OpenMenu::CharGen(m) => &mut **m,
            OpenMenu::TextEdit(m) => &mut **m,
            OpenMenu::SleepWait(m) => &mut **m,
            OpenMenu::Vigor(m) => &mut m.menu,
            OpenMenu::Start(m) => &mut m.menu,
            OpenMenu::Hacking(m) => &mut m.menu,
            OpenMenu::Computers(m) => &mut m.menu,
            OpenMenu::Tutorial(m) => &mut **m,
        }
    }

    fn tile(&self) -> TileId {
        match self {
            OpenMenu::Message(m) => m.menu,
            OpenMenu::Dialog(m) => m.menu,
            OpenMenu::Container(c) => c.menu.menu,
            OpenMenu::Barter(b) => b.menu.menu,
            OpenMenu::Recipe(r) => r.menu.menu,
            OpenMenu::Repair(r) => r.menu.menu,
            OpenMenu::CompanionWheel(w) => w.menu.menu,
            OpenMenu::Caravan(c) => c.menu.menu,
            OpenMenu::Slots(s) => s.menu.menu,
            OpenMenu::Blackjack(b) => b.menu.menu,
            OpenMenu::Roulette(r) => r.menu.menu,
            OpenMenu::Quantity(m) => m.menu,
            OpenMenu::LevelUp(m) => m.menu,
            OpenMenu::Traits(m) => m.menu,
            OpenMenu::CharGen(m) => m.menu,
            OpenMenu::TextEdit(m) => m.menu,
            OpenMenu::SleepWait(m) => m.menu,
            OpenMenu::Vigor(m) => m.menu.menu,
            OpenMenu::Start(m) => m.menu.menu,
            OpenMenu::Hacking(m) => m.menu.menu,
            OpenMenu::Computers(m) => m.menu.menu,
            OpenMenu::Tutorial(m) => m.menu,
        }
    }

    /// A service menu: one whose making hides the conversation
    /// (`00763ff0`: the barter menu `0072d250`, the recipe menu
    /// `00726ff0`) and whose closing brings it back (`007640a0`: `0072d6d0`,
    /// `00727430`). (A companion's trade, the repair menu and the face menu
    /// do too; they aren't shown here. Containers in mode 1 don't.)
    fn is_service(&self) -> bool {
        matches!(self, OpenMenu::Barter(_) | OpenMenu::Recipe(_))
    }

    /// A menu that draws a 3D scene into the HUD's picture, under the
    /// menus' tiles: the Vigor Tester's machine (`vigor::draw`), the Caravan
    /// table (`caravan_table`), the casino games' machine and tables
    /// (`casino_scene`). (The rendered terminal lays its model on the HUD's
    /// picture as one of its tiles, `rendered_terminal`, so isn't one.)
    fn draws_scene(&self) -> bool {
        matches!(
            self,
            OpenMenu::Vigor(_)
                | OpenMenu::Caravan(_)
                | OpenMenu::Slots(_)
                | OpenMenu::Blackjack(_)
                | OpenMenu::Roulette(_)
        )
    }

    fn closed(&self) -> bool {
        match self {
            OpenMenu::Message(m) => m.closed,
            OpenMenu::Dialog(m) => m.closed,
            OpenMenu::Container(c) => c.menu.closed,
            OpenMenu::Barter(b) => b.menu.closed,
            OpenMenu::Recipe(r) => r.menu.closed,
            OpenMenu::Repair(r) => r.menu.closed,
            OpenMenu::CompanionWheel(w) => w.menu.closed,
            OpenMenu::Caravan(c) => c.closed,
            OpenMenu::Slots(s) => s.closed,
            OpenMenu::Blackjack(b) => b.closed,
            OpenMenu::Roulette(r) => r.closed,
            OpenMenu::Quantity(m) => m.closed,
            OpenMenu::LevelUp(m) => m.closed,
            OpenMenu::Traits(m) => m.closed,
            OpenMenu::CharGen(m) => m.closed,
            OpenMenu::TextEdit(m) => m.closed,
            OpenMenu::SleepWait(m) => m.closed,
            OpenMenu::Vigor(m) => m.menu.closed,
            // Its requests (the load asked for, back to the game) are
            // carried out first (`start::frame`).
            OpenMenu::Start(m) => m.menu.closed && m.menu.requests.is_empty(),
            OpenMenu::Hacking(m) => m.menu.closed,
            OpenMenu::Computers(m) => m.menu.closed,
            OpenMenu::Tutorial(m) => m.closed,
        }
    }
}

pub struct GameMenusPlugin;

impl Plugin for GameMenusPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameMenus>()
            .init_resource::<MenuDraw>()
            .init_resource::<asks::PipboyAsks>()
            .init_resource::<StartMenu>()
            .init_resource::<FixedPointer>()
            .init_resource::<start::GameSettings>()
            .init_resource::<start::SaveFiles>()
            .init_resource::<FixedClicks>()
            .init_resource::<FixedKeys>()
            .init_resource::<AnswerBoxes>()
            .init_resource::<hacking::HackingSounds>()
            .init_resource::<casino::CasinoLock>()
            .init_resource::<companion_wheel::WheelVoices>()
            .add_systems(
                Update,
                (
                    start_menu,
                    escape_opens_start_menu,
                    open_menus,
                    run_tutorials,
                    run_open_menus,
                    start_menu_frame,
                    draw_menus,
                )
                    .chain()
                    // Kept: the game's menus have the input before the
                    // viewer's own (both in the interface idle's set,
                    // `crate::frame_order`).
                    .before(crate::menus::run_menus)
                    .in_set(crate::frame_order::IdleSet),
            )
            // Kept, inside the interface idle's set: each after the menus it draws
            // or voices have run this frame.
            .add_systems(
                Update,
                (
                    vigor::draw.after(run_open_menus),
                    compose_hud_over_scene.after(crate::menus::run_menus),
                    hacking::play_sounds.after(run_open_menus),
                    companion_wheel::play_voices.after(run_open_menus),
                )
                    .in_set(crate::frame_order::IdleSet),
            );
    }
}

/// How the HUD's camera writes its picture this frame: over a menu's 3D
/// scene (drawn into the same picture first, by the scene's own camera at
/// order −5) it blends its tiles over it; otherwise it writes the picture
/// as it is. The lockpicking menu (`lockpick`, not one of these menus)
/// draws its scene and pictures into the HUD's picture the same way.
fn hud_output(open: &[OpenMenu], lockpicking: bool) -> CameraOutputMode {
    if lockpicking || open.iter().any(OpenMenu::draws_scene) {
        CameraOutputMode::Write {
            blend_state: Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING),
            clear_color: ClearColorConfig::None,
        }
    } else {
        CameraOutputMode::default()
    }
}

fn blends(mode: &CameraOutputMode) -> bool {
    matches!(
        mode,
        CameraOutputMode::Write {
            blend_state: Some(_),
            ..
        }
    )
}

/// The one place that sets the HUD camera's output, for every menu that
/// draws a 3D scene under the menus. (Each menu used to set it itself, and
/// the Caravan table's "not open, so stop blending" undid the Vigor
/// Tester's every frame: the HUD's empty picture replaced the machine, so
/// the tester's menu worked but couldn't be seen.)
fn compose_hud_over_scene(
    menus: Res<GameMenus>,
    queue: Res<crate::menus::Menus>,
    mut cameras: Query<&mut Camera, With<crate::hud::HudCamera>>,
) {
    let wanted = hud_output(
        menus.screen.as_deref().map_or(&[], |s| &s.open),
        queue.lockpicking,
    );
    for mut camera in &mut cameras {
        if blends(&camera.output_mode) != blends(&wanted) {
            camera.output_mode = wanted;
        }
    }
}

/// What `world::menu_background` reads from the game's menus here: the
/// start menu up as the pause menu (`004a4040`) and as the main menu
/// (`0070edf0`, without its pause flag), and whether a tile is named
/// "Player Name Entry Menu" (`00718ab0`, `00a03da0`).
pub(crate) fn background_facts(menus: &GameMenus) -> (bool, bool, bool) {
    let Some(screen) = menus.screen.as_deref() else {
        return (false, false, false);
    };
    let pause = tutorial::held(&screen.open);
    let start = screen.open.iter().any(|m| matches!(m, OpenMenu::Start(_)));
    let naming = screen
        .ui
        .find_below(screen.ui.screen, "Player Name Entry Menu")
        .is_some();
    (pause, start && !pause, naming)
}
/// Which button `--answer-boxes` gives a box of `buttons` buttons: the only
/// one, else the next choice given (taken), else none (left open).
fn box_choice(buttons: usize, choices: &mut std::collections::VecDeque<usize>) -> Option<usize> {
    match buttons {
        0 => None,
        1 => Some(0),
        _ => choices.pop_front(),
    }
}

/// `--answer-boxes`' step on the menu on top: a message box answered with
/// a button (a click on it, `007aa070`) by [`box_choice`], a tutorial box
/// closed (its close button, `007e8db0`), the name entry's text accepted
/// (Enter, `007e6620`).
fn answer_box(ui: &mut ui::Ui, top: &mut OpenMenu, answers: &mut AnswerBoxes, now: f64) {
    match top {
        OpenMenu::Message(m) if !m.closed => {
            let Some(b) = m.queue.front() else {
                return;
            };
            let what = format!("\"{}\" {:?}", b.text.replace('\n', " "), b.buttons);
            // Buttons by their index among the box's own (a blank one
            // keeps its index but isn't shown, `007a92e0`).
            let shown: Vec<usize> = (0..b.buttons.len())
                .filter(|&i| !b.buttons[i].is_empty())
                .collect();
            let first = b.first_number;
            if shown.len() > 1 && answers.choices.is_empty() {
                if answers.waiting.as_deref() != Some(what.as_str()) {
                    println!("--answer-boxes: {what} waits for a choice (--box-answers).");
                    answers.waiting = Some(what);
                }
                return;
            }
            let Some(i) = box_choice(shown.len(), &mut answers.choices) else {
                return;
            };
            let i = if shown.len() == 1 { shown[0] } else { i };
            let value = first + i as i32;
            let Some(tile) = m
                .list
                .items
                .iter()
                .find(|item| item.value == value)
                .map(|item| item.tile)
            else {
                println!("--answer-boxes: {what} has no button {i}.");
                return;
            };
            println!("--answer-boxes: {what} answered with button {i}.");
            answers.waiting = None;
            m.click(ui, 7, Some(tile), now);
        }
        OpenMenu::Tutorial(m) if !m.closed => {
            println!("--answer-boxes: a tutorial box closed.");
            m.click(ui, ui::menus::tutorial::tile::CLOSE as i32, None, now);
        }
        OpenMenu::TextEdit(m) if !m.closed => {
            if m.key(ui, ui::menu::key::ENTER, now) {
                println!("--answer-boxes: the name entry accepted.");
            }
        }
        _ => {}
    }
}

/// `--menu-pointer X,Y`: the menus' pointer held at this pixel of a 1920 ×
/// 1080 picture (for screenshots, which have no mouse).
#[derive(Resource, Default)]
pub struct FixedPointer(pub Option<(f32, f32)>);

/// `--menu-click S,...`: left clicks at these seconds (pressed one frame,
/// let go the next), for testing with `FixedPointer`.
#[derive(Resource, Default)]
pub struct FixedClicks {
    pub at: Vec<f64>,
    pub release: bool,
}

/// `--menu-keys S:K,...`: keys typed into the top menu at these seconds
/// (a character, or left, right, up, down), for testing.
#[derive(Resource, Default)]
pub struct FixedKeys(pub Vec<(f64, String)>);

/// `--answer-boxes` (a test aid for scripted routes, `scripts/acceptance.ps1`):
/// the prompts on top, once shown, answered by rule as a player would. A
/// message box with one button (an information box's OK) is answered with
/// it; one with more buttons with the next of `--box-answers` (button
/// indices in the box's own order, one per such box in turn), or left open
/// when none is left; a tutorial box is closed; the name entry is accepted
/// with Enter (the name in it). The prompts themselves open as the game
/// opens them.
#[derive(Resource, Default)]
pub struct AnswerBoxes {
    pub on: bool,
    /// `--box-answers`: the choices still to give, in order.
    pub choices: std::collections::VecDeque<usize>,
    /// The box last reported as waiting for a choice (said once).
    waiting: Option<String>,
}

impl AnswerBoxes {
    pub fn new(on: bool, choices: Vec<usize>) -> AnswerBoxes {
        AnswerBoxes {
            on,
            choices: choices.into(),
            waiting: None,
        }
    }
}

/// `--open-menu NAME[:ID]`: a menu to open once the place has loaded, for
/// testing.
#[derive(Resource, Default)]
pub struct StartMenu(pub Option<String>);

/// A form by editor ID or hexadecimal form ID.
fn form_named(order: &esm::LoadOrder, id: &str) -> Option<esm::FormId> {
    order.form_by_editor_id(id).or_else(|| {
        u32::from_str_radix(id.trim_start_matches("0x"), 16)
            .ok()
            .map(esm::FormId)
            .filter(|f| order.get(*f).is_some())
    })
}

/// A reference's name (its base's `FULL`).
fn reference_name(order: &esm::LoadOrder, reference: esm::FormId) -> String {
    world::scripting::base_of(order, reference)
        .and_then(|b| order.get(b))
        .and_then(|r| r.record().ok())
        .and_then(|r| r.full_name())
        .unwrap_or_default()
}

/// Opens the `--open-menu` menu once the player is in the place.
#[allow(clippy::too_many_arguments)]
fn start_menu(
    game: Res<GameFiles>,
    mut start: ResMut<StartMenu>,
    mut queue: ResMut<crate::menus::Menus>,
    player: Res<crate::walk::Player>,
    mut menus: ResMut<GameMenus>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    commands: Res<crate::scripts::StartCommands>,
) {
    // After `--run`'s lines (a menu opened first would hold them back).
    if !player.ready || !commands.0.is_empty() {
        return;
    }
    let Some(asked) = start.0.take() else {
        return;
    };
    // The player goes up a level now (`world::experience::level_up`);
    // `levelup:perks` then gives the points and goes on to the perks.
    if asked == "levelup" || asked == "levelup:perks" {
        let up = world::experience::level_up(&game.0.order, &mut state.0);
        println!("--open-menu: level {}.", up.level);
        menus.levelup_to_perks = asked.ends_with(":perks");
        queue.push(crate::menus::Menu::LevelUp(up));
        return;
    }
    let order = &game.0.order;
    let (name, id) = asked.split_once(':').unwrap_or((asked.as_str(), ""));
    // The sleep/wait menu in either mode (as `ShowSleepWaitMenu` opens it,
    // without the refusals).
    if name == "sleep" || name == "wait" {
        println!("--open-menu: the {name} menu.");
        menus.sleepwait_hours = id.parse::<u32>().ok().filter(|n| (1..=24).contains(n));
        queue.push(crate::menus::Menu::SleepWait {
            sleep: name == "sleep",
        });
        return;
    }
    // "How many?" on its own (the game only opens it from other menus).
    if name == "quantity" {
        let most = id.parse::<i32>().unwrap_or(20);
        let size = window_size(&windows);
        if let Some(screen) = size.and_then(|s| screen(&mut menus, &game.0, s)) {
            println!("--open-menu: how many, up to {most}.");
            container::open_quantity(screen, &game.0, most);
            queue.game_open = true;
        }
        return;
    }
    let form = form_named(order, id);
    let request = match (name, form) {
        ("container", Some(r)) => crate::menus::Menu::Container(r, reference_name(order, r)),
        ("barter", Some(r)) => crate::menus::Menu::Barter(r),
        // A recipe category's crafting menu for the player, as
        // `ShowRecipeMenu` would open it.
        ("recipes", Some(r)) => crate::menus::Menu::Recipe {
            actor: world::dialogue::PLAYER_REF,
            category: r,
        },
        // A vendor's repairs, as `ShowRepairMenu` on them would open them.
        ("repair", Some(r)) => crate::menus::Menu::RepairServices(r),
        // A companion's things, as `OpenTeammateContainer` would open them.
        ("teammate", Some(r)) => crate::menus::Menu::Teammate(r),
        // A companion's wheel, as using them would bring it up.
        ("wheel", Some(r)) => crate::menus::Menu::CompanionWheel(r),
        // A placed terminal's own screen, as getting in would open it.
        ("terminal", Some(r)) => {
            crate::menus::Menu::Terminal(world::scripting::base_of(order, r).unwrap_or(r), r)
        }
        // A placed terminal's hacking menu, as using it would open it.
        ("hacking", Some(r)) => {
            crate::menus::Menu::Hacking(world::scripting::base_of(order, r).unwrap_or(r), r)
        }
        _ => {
            println!("--open-menu: don't know how to open '{asked}'.");
            return;
        }
    };
    println!("--open-menu: opening {asked}.");
    queue.push(request);
}

/// Every text game setting (`GMST` named `s…`), for `&-sName;`.
fn text_settings(order: &esm::LoadOrder) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for rr in order.records_of_type(esm::FourCC::new(b"GMST")) {
        let Ok(record) = rr.record() else {
            continue;
        };
        let Some(name) = record.get(esm::sig::EDID).map(|s| s.zstring()) else {
            continue;
        };
        if name.starts_with('s') {
            if let Some(text) = record.get(esm::sig::DATA).map(|s| s.zstring()) {
                out.insert(name, text);
            }
        }
    }
    out
}

/// The cursor's picture, a game setting (`00f6ccc0`).
const CURSOR_DEFAULT: &str = "Interface\\Icons\\Misc\\cursor.dds";

impl Screen {
    fn build(game: &Game, size: UVec2) -> Screen {
        let mut read = |p: &str| game.assets.read(p).ok().flatten();
        let ini = |s: &str, k: &str| game.settings.get(s, k).map(str::to_string);
        let mut ui = ui::game::new_ui(&mut read, &ini, text_settings(&game.order), size.x, size.y);
        for w in &ui.warnings {
            println!("  Menus: {w}");
        }
        // The cursor (`0070b240`): an image named "Cursor", depth 2000, 32 ×
        // 32, its picture `sCursorFilename`, white, hidden until the
        // menus need it.
        let file = ui
            .setting_text("sCursorFilename")
            .unwrap_or_else(|| CURSOR_DEFAULT.to_string());
        let cursor = ui
            .load_menu(
                format!(
                    "<image name=\"Cursor\"><depth>2000</depth><width>32</width><height>32</height><filename>{file}</filename><x>0</x><y>0</y><alpha>255</alpha><red>255</red><green>255</green><blue>255</blue></image>"
                )
                .as_bytes(),
                &mut |_| None,
            )
            .ok();
        // The fader (`00700960`): a quad over the whole screen with
        // `Data\Textures\Interface\Faders\Black.dds`, under the menus;
        // hidden until a sleep fades the screen.
        let fader = ui
            .load_menu(
                "<image name=\"Fader\"><depth>1</depth><width><copy src=\"screen()\" trait=\"width\"/></width><height><copy src=\"screen()\" trait=\"height\"/></height><filename>Interface\\Faders\\Black.dds</filename><zoom>-1</zoom><x>0</x><y>0</y><alpha>0</alpha><red>255</red><green>255</green><blue>255</blue><visible>&false;</visible></image>"
                    .as_bytes(),
                &mut |_| None,
            )
            .ok();
        Screen {
            ui,
            interface: Interface::default(),
            open: Vec::new(),
            fading: Vec::new(),
            fades: ui::fade::Fades::default(),
            size,
            cursor,
            fade: None,
            fader,
            rest_down: false,
            quantity_owner: None,
            pointer: None,
            mouse_move: (0.0, 0.0),
            terminal_left: false,
            sizes: HashMap::new(),
            atlases: HashMap::new(),
            models: HashMap::new(),
        }
    }

    /// The menu tiles open, bottom first.
    fn menu_tiles(&self) -> Vec<TileId> {
        self.open.iter().map(OpenMenu::tile).collect()
    }

    /// Whether the game's menus hold the game: one open, or one still
    /// fading out (the game stays in menu mode until the fades end,
    /// `00711ea0`).
    pub fn busy(&self) -> bool {
        !self.open.is_empty() || !self.fading.is_empty()
    }

    /// Menus opened since the last look start fading in (every menu here
    /// opens with `Menu::PrepForVisibility(0)`, `00a1dc20`: its tile
    /// hidden and `StartFadeIn`; only the Pip-Boy's pages and the HUD
    /// open at once). `dt` is the frame's seconds.
    fn show_new(&mut self, dt: f32) {
        for tile in self.menu_tiles() {
            if !self.fades.knows(tile) {
                self.fades.show(&mut self.ui, tile, false, dt);
                let secs = ui::fade::seconds(&mut self.ui, tile);
                println!("Menu {} fading in ({secs} s).", self.ui.tiles[tile].name);
            }
        }
    }

    /// A closed menu leaves the screen: it fades out
    /// (`Menu::StartFadeOut`, `00a1d910`), hides at once and goes at the
    /// frame's end (`InstantFadeOut`, `00a1d9e0`), or, `Close::Now`, goes
    /// now (the terminal's power button deletes it, `007ffd50`; a menu
    /// closed before it was ever shown, whose `StartFadeOut` does nothing
    /// in the game, too).
    fn close(&mut self, m: OpenMenu, how: Close, dt: f32) {
        let tile = m.tile();
        self.interface.let_go_of(&self.ui, tile);
        let fading = match how {
            Close::Fade => self.fades.start_fade_out(&mut self.ui, tile, dt),
            Close::Instant => {
                self.fades.instant_fade_out(&mut self.ui, tile);
                true
            }
            Close::Now => false,
        };
        if fading {
            let secs = ui::fade::seconds(&mut self.ui, tile);
            match how {
                Close::Instant => println!("Menu {} gone at once.", self.ui.tiles[tile].name),
                _ => println!("Menu {} fading out ({secs} s).", self.ui.tiles[tile].name),
            }
            self.fading.push(m);
        } else {
            self.fades.forget(tile);
            self.ui.detach(tile);
        }
    }

    /// The frame's end for the menus' fades (`00711ea0`): what they're
    /// drawn with this frame, and the faded-out menus taken away.
    fn end_fades(&mut self) {
        let tiles: Vec<TileId> = self
            .fading
            .iter()
            .chain(self.open.iter())
            .map(OpenMenu::tile)
            .collect();
        for (tile, _) in self.fades.frame(&mut self.ui, &tiles) {
            // Kept menus (not marked to leave the stack) would only be
            // hidden; every closed menu here is let go of either way.
            if let Some(i) = self.fading.iter().position(|m| m.tile() == tile) {
                println!("Menu {} faded out.", self.ui.tiles[tile].name);
                self.fading.remove(i);
                self.fades.forget(tile);
                self.ui.detach(tile);
            }
        }
    }

    /// Loads a menu file for a menu's code (`ui::menu::load`), above the
    /// menus open.
    pub fn load(
        &mut self,
        game: &Game,
        path: &str,
        code: &mut dyn MenuCode,
    ) -> Result<TileId, String> {
        // Every menu the interface has, those fading out too (`00a1dfb0`).
        let open: Vec<TileId> = self
            .fading
            .iter()
            .chain(self.open.iter())
            .map(OpenMenu::tile)
            .collect();
        let depth = ui::menu::next_depth(&mut self.ui, &open);
        let mut read = |p: &str| game.assets.read(p).ok().flatten();
        let tile = ui::menu::load(&mut self.ui, &mut read, path, code, depth)?;
        for w in self.ui.warnings.drain(..) {
            println!("  {path}: {w}");
        }
        Ok(tile)
    }

    /// The message menu, if it's open.
    pub fn message(&mut self) -> Option<&mut ui::menus::message::MessageMenu> {
        self.open.iter_mut().find_map(|m| match m {
            OpenMenu::Message(m) => Some(m),
            _ => None,
        })
    }
}

impl GameMenus {
    /// The menus, once built for the window (`None` before the first frame
    /// or when the game's menu files can't be read).
    pub fn screen(&mut self) -> Option<&mut Screen> {
        self.screen.as_deref_mut()
    }
}

/// Texture sizes and atlases from the game's files.
struct Files<'a> {
    game: &'a Game,
    sizes: &'a mut HashMap<String, Option<(u32, u32)>>,
    atlases: &'a mut HashMap<String, Option<ui::Atlas>>,
}

impl Textures for Files<'_> {
    fn size(&mut self, path: &str) -> Option<(u32, u32)> {
        let game = self.game;
        *self.sizes.entry(path.to_string()).or_insert_with(|| {
            let bytes = game.assets.read(path).ok().flatten()?;
            let t = TextureData::from_dds(path, bytes).ok()?;
            Some((t.width, t.height))
        })
    }

    fn atlas(&mut self, path: &str) -> Option<ui::Atlas> {
        let game = self.game;
        self.atlases
            .entry(path.to_string())
            .or_insert_with(|| {
                let bytes = game.assets.read(path).ok().flatten()?;
                Some(ui::Atlas::parse(&String::from_utf8_lossy(&bytes)))
            })
            .clone()
    }
}

/// The window's size in pixels.
fn window_size(windows: &Query<&Window, With<PrimaryWindow>>) -> Option<UVec2> {
    let w = windows.single().ok()?;
    let size = UVec2::new(w.physical_width(), w.physical_height());
    (size.x > 0 && size.y > 0).then_some(size)
}

/// The menus built for the window (again when its size changes).
fn screen<'a>(menus: &'a mut GameMenus, game: &Game, size: UVec2) -> Option<&'a mut Screen> {
    if menus.failed {
        return None;
    }
    if menus.screen.as_ref().is_none_or(|s| s.size != size) {
        if menus.screen.as_ref().is_some_and(|s| s.busy()) {
            // Menus open (or fading out) keep their layout until they
            // close.
        } else {
            let built = Screen::build(game, size);
            if built.ui.fonts.iter().all(Option::is_none) {
                println!("The game's menus can't be shown: no fonts found.");
                menus.failed = true;
                return None;
            }
            menus.screen = Some(Box::new(built));
        }
    }
    menus.screen.as_deref_mut()
}

/// Whether the game's own menus show a request (the rest are the viewer's
/// panel, `menus`).
pub fn takes(m: &crate::menus::Menu) -> bool {
    message::takes(m)
        || container::takes(m)
        || barter::takes(m)
        || recipe::takes(m)
        || repair::takes(m)
        || companion_wheel::takes(m)
        || caravan::takes(m)
        || slots::takes(m)
        || blackjack::takes(m)
        || roulette::takes(m)
        || levelup::takes(m)
        || traits::takes(m)
        || chargen::takes(m)
        || textedit::takes(m)
        || sleepwait::takes(m)
        || vigor::takes(m)
        || hacking::takes(m)
        || computers::takes(m)
        || matches!(m, crate::menus::Menu::Tutorial(_))
}

/// The world's requests become menus.
#[allow(clippy::too_many_arguments)]
fn open_menus(
    game: Res<GameFiles>,
    mut menus: ResMut<GameMenus>,
    mut queue: ResMut<crate::menus::Menus>,
    mut player: ResMut<crate::walk::Player>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    mut sounds: ResMut<crate::sounds::SoundRequests>,
    windows: Query<&Window, With<PrimaryWindow>>,
    time: Res<Time<bevy::time::Real>>,
    mut asks: ResMut<asks::PipboyAsks>,
    hacking_sounds: Res<hacking::HackingSounds>,
    scripts: Res<crate::scripts::Scripts>,
) {
    let Some(size) = window_size(&windows) else {
        return;
    };
    let wanted = menus.levelup_to_perks;
    let mut to_perks = wanted;
    let sleepwait_hours = menus.sleepwait_hours.take();
    let Some(screen) = screen(&mut menus, &game.0, size) else {
        queue.game_ready = false;
        return;
    };
    queue.game_ready = true;
    while let Some(request) = queue.take_next(takes) {
        let before = screen.open.len();
        if message::takes(&request) {
            message::open(screen, &game.0, request);
        } else if let crate::menus::Menu::Tutorial(form) = request {
            tutorial::show_form(screen, &game.0, form);
        } else if hacking::takes(&request) {
            hacking::open(screen, &game.0, &mut state.0, request, &hacking_sounds);
        } else if computers::takes(&request) {
            computers::open(screen, &game.0, &mut state.0, request);
        } else if vigor::takes(&request) {
            sounds
                .0
                .extend(vigor::open(screen, &game.0, &mut state.0, request));
        } else if traits::takes(&request) {
            traits::open(screen, &game.0, &mut state.0, request);
        } else if chargen::takes(&request) {
            chargen::open(screen, &game.0, &mut state.0, request);
        } else if textedit::takes(&request) {
            textedit::open(screen, &game.0, &state.0, time.elapsed_secs_f64() * 1000.0);
        } else if sleepwait::takes(&request) {
            sounds
                .0
                .extend(sleepwait::open(screen, &game.0, &state.0, request));
            if let Some(hours) = sleepwait_hours {
                sleepwait::choose_and_wait(screen, hours);
            }
        } else if levelup::takes(&request) {
            sounds
                .0
                .extend(levelup::open(screen, &game.0, &mut state.0, request));
            if std::mem::take(&mut to_perks) {
                levelup::give_points_and_continue(screen, &game.0, &mut state.0);
            }
        } else if barter::takes(&request) {
            sounds
                .0
                .extend(barter::open(screen, &game.0, &mut state.0, request));
        } else if recipe::takes(&request) {
            recipe::open(screen, &game.0, &mut state.0, request);
        } else if repair::takes(&request) {
            repair::open(screen, &game.0, &mut state.0, request);
        } else if companion_wheel::takes(&request) {
            companion_wheel::open(screen, &game.0, &scripts.0, &mut state.0, request);
        } else if caravan::takes(&request) {
            caravan::open(screen, &game.0, &mut state.0, request);
        } else if slots::takes(&request) {
            sounds
                .0
                .extend(slots::open(screen, &game.0, &mut state.0, request));
        } else if blackjack::takes(&request) {
            sounds
                .0
                .extend(blackjack::open(screen, &game.0, &mut state.0, request));
        } else if roulette::takes(&request) {
            sounds
                .0
                .extend(roulette::open(screen, &game.0, &mut state.0, request));
        } else {
            sounds
                .0
                .extend(container::open(screen, &game.0, &mut state.0, request));
        }
        // A service menu made: the conversation fades out under it.
        if screen.open.len() > before && screen.open.last().is_some_and(OpenMenu::is_service) {
            dialog::service_opened(screen, time.delta_secs());
        }
        player.ready = false;
    }
    // The Pip-Boy's questions (over it, so the player isn't ready anyway).
    asks::open(screen, &game.0, &mut asks);
    queue.game_open = screen.busy();
    if wanted && !to_perks {
        menus.levelup_to_perks = false;
    }
}

/// The tutorial manager's update (`007182e0`, in the interface manager's
/// update): a message it picks opens over the menu it's for
/// (`tutorial`).
fn run_tutorials(
    game: Res<GameFiles>,
    mut menus: ResMut<GameMenus>,
    mut queue: ResMut<crate::menus::Menus>,
    mut state: ResMut<crate::dialogue::DialogueState>,
    time: Res<Time<bevy::time::Real>>,
    vats: Option<Res<crate::vats::Vats>>,
    pipboy: Option<Res<crate::pipboy::Pipboy>>,
) {
    let Some(screen) = menus.screen.as_deref_mut() else {
        return;
    };
    let now_ms = time.elapsed_secs_f64() * 1000.0;
    // The viewer's own menus on top when none of the game's is: the
    // lockpicking menu, V.A.T.S.'s, the Pip-Boy's.
    let other = if queue.lockpicking {
        Some(world::tutorial::menu::LOCKPICK)
    } else if vats.as_ref().is_some_and(|v| v.in_menu()) {
        Some(world::tutorial::menu::VATS)
    } else {
        pipboy.as_ref().and_then(|p| p.top_class())
    };
    tutorial::update(screen, &game.0, &mut state.0, now_ms, other);
    queue.game_open = !screen.open.is_empty();
}

/// What the menus need each frame from Bevy: the pointer, buttons, wheel
/// and keys.
#[derive(bevy::ecs::system::SystemParam)]
pub struct MenuInput<'w, 's> {
    windows: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    mouse: Res<'w, ButtonInput<MouseButton>>,
    scroll: Res<'w, AccumulatedMouseScroll>,
    motion: Res<'w, AccumulatedMouseMotion>,
    keys: ResMut<'w, ButtonInput<KeyCode>>,
    typed: EventReader<'w, 's, KeyboardInput>,
    time: Res<'w, Time<bevy::time::Real>>,
    fixed: Res<'w, FixedPointer>,
    clicks: ResMut<'w, FixedClicks>,
    fixed_keys: ResMut<'w, FixedKeys>,
    answer_boxes: ResMut<'w, AnswerBoxes>,
    pads: Query<'w, 's, &'static Gamepad>,
    /// Terminals drawn on the terminal's screen: the pointer goes through
    /// it.
    rendered: ResMut<'w, crate::rendered_terminal::RenderedTerminal>,
    hud: Option<Res<'w, crate::hud::HudLayer>>,
}

/// A key as the game's interface turns it into a menu code (`007154b0`):
/// characters as themselves, Backspace, the arrows, Home, End, Delete,
/// Enter, Page Up and Page Down as the interface's own codes.
fn key_code(key: &Key) -> Option<u32> {
    use ui::menu::key;
    Some(match key {
        Key::Character(c) => {
            let ch = c.chars().next()?;
            if !ch.is_ascii() {
                return None;
            }
            ch as u32
        }
        Key::Space => u32::from(b' '),
        Key::Backspace => key::BACKSPACE,
        Key::ArrowLeft => key::LEFT,
        Key::ArrowRight => key::RIGHT,
        Key::ArrowUp => key::UP,
        Key::ArrowDown => key::DOWN,
        Key::Home => key::HOME,
        Key::End => key::END,
        Key::Delete => key::DELETE,
        Key::Enter => key::ENTER,
        Key::PageUp => key::PAGE_UP,
        Key::PageDown => key::PAGE_DOWN,
        _ => return None,
    })
}

/// The open menus get the pointer and the keys, then hand what the player
/// did back to the game.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_open_menus(
    game: Res<GameFiles>,
    mut menus: ResMut<GameMenus>,
    mut queue: ResMut<crate::menus::Menus>,
    mut input: MenuInput,
    mut state: ResMut<crate::dialogue::DialogueState>,
    mut player: ResMut<crate::walk::Player>,
    conversation: Res<crate::dialogue::Conversation>,
    mut sounds: ResMut<crate::sounds::SoundRequests>,
    scripts: Res<crate::scripts::Scripts>,
    mut asks: ResMut<asks::PipboyAsks>,
    mut hacking_sounds: ResMut<hacking::HackingSounds>,
    mut hud_messages: ResMut<crate::hud::HudMessages>,
    mut wheel_voices: ResMut<companion_wheel::WheelVoices>,
    mut casino_lock: ResMut<casino::CasinoLock>,
    mut library: Option<ResMut<crate::anim_library::AnimLibrary>>,
) {
    let Some(screen) = menus.screen.as_deref_mut() else {
        input.typed.clear();
        return;
    };
    let dt = input.time.delta_secs();
    // Before the menus run, the fades move on (`0070b8f0` → `00716320`);
    // menus opened since start fading in.
    screen.fades.update(dt);
    screen.show_new(dt);
    if screen.open.is_empty() {
        input.typed.clear();
        // Menus still fading out hold the game until they're gone.
        if !screen.fading.is_empty() {
            screen.end_fades();
            queue.game_open = screen.busy();
            if !screen.busy() && conversation.0.is_none() && !queue.pipboy {
                player.ready = true;
            }
        }
        return;
    }
    let now = input.time.elapsed_secs_f64();
    // The pad's left stick (XInput's ±32767 from Bevy's −1..1) and the
    // mouse held, for Caravan.
    let caravan_input = {
        let axis = |a: GamepadAxis| {
            let v = input.pads.iter().find_map(|g| g.get(a)).unwrap_or(0.0);
            (v * 32767.0).round() as i32
        };
        caravan::PadInput {
            pad: !input.pads.is_empty(),
            stick: [
                world::caravan::menu::stick_axis(axis(GamepadAxis::LeftStickX)),
                -world::caravan::menu::stick_axis(axis(GamepadAxis::LeftStickY)),
            ],
            mouse_held: input.mouse.pressed(MouseButton::Left),
        }
    };
    // A rendered menu is up (the terminal's model: `00707af0`): the
    // message box and "how many?" close without a fade.
    let rendered_up = input.rendered.prepare(&game.0, input.hud.is_some())
        && screen
            .open
            .iter()
            .chain(screen.fading.iter())
            .any(|m| matches!(m, OpenMenu::Hacking(_) | OpenMenu::Computers(_)));
    let mut power_off_menu = None;
    dialog::update(screen, &game.0.order, dt);
    container::update(screen, now * 1000.0);
    textedit::update(screen, now * 1000.0);
    vigor::update(screen, dt);
    companion_wheel::update(screen, &mut wheel_voices, now * 1000.0);
    companion_wheel::stick(screen, input.pads.iter().next().map(|p| p.left_stick()));
    // The Rest control (T) this frame, for the sleep/wait menu.
    let mut rest_pressed = false;
    {
        let Screen {
            ui,
            interface,
            open,
            rest_down,
            pointer: here,
            mouse_move,
            fades,
            ..
        } = &mut *screen;
        // The mouse's raw movement (roulette's cursor, `00a239e0`).
        *mouse_move = (input.motion.delta.x, input.motion.delta.y);
        let Some(top) = open.last_mut() else {
            return;
        };
        let menu = top.tile();
        // The pointer reaches a menu only while it's shown, not while it
        // fades in (`0070c4a0`); the keys do.
        let shown = fades.takes_pointer(menu);
        if !shown {
            interface.let_go_of(ui, menu);
        }
        // `--answer-boxes`: the box on top answered once it's shown.
        if input.answer_boxes.on && shown {
            answer_box(ui, top, &mut input.answer_boxes, now);
        }
        // `--menu-click`: pressed this frame, let go the next.
        let release = std::mem::take(&mut input.clicks.release);
        let press = {
            let due = input.clicks.at.iter().position(|&t| t <= now);
            due.map(|i| input.clicks.at.remove(i)).is_some()
        };
        input.clicks.release = press;
        // The pointer, in menu units.
        let k = ui.screen_size.resolution_converter();
        let window = input.windows.single().ok();
        let rendered = matches!(top, OpenMenu::Hacking(_) | OpenMenu::Computers(_))
            && input.rendered.prepare(&game.0, input.hud.is_some());
        let mut power_button = false;
        let pointer = if rendered {
            // On the terminal's screen (`007fb790`): where the ray through
            // the pointer meets it; `--menu-pointer` is a 1920 × 1080
            // picture's pixels, as elsewhere.
            let size = window.map_or(UVec2::new(1920, 1080), |w| {
                UVec2::new(w.physical_width(), w.physical_height())
            });
            let at = match input.fixed.0 {
                Some((x, y)) => Some(Vec2::new(
                    x * size.x as f32 / 1920.0,
                    y * size.y as f32 / 1080.0,
                )),
                None => window.and_then(|w| w.physical_cursor_position()),
            };
            let click = input.mouse.just_pressed(MouseButton::Left) || press;
            at.and_then(|p| input.rendered.hit(&game.0, p, size, click))
                .map(|hit| {
                    power_button = click && hit == cellview::rendered_terminal::Hit::Exit;
                    crate::rendered_terminal::RenderedTerminal::menu_pointer(hit)
                })
        } else {
            match input.fixed.0 {
                // A 1920 × 1080 picture's pixels: the screen is 960 units
                // high.
                Some((x, y)) => Some((x * 960.0 / 1080.0, y * 960.0 / 1080.0)),
                None => {
                    window.and_then(|w| w.physical_cursor_position().map(|p| (p.x * k, p.y * k)))
                }
            }
        };
        *here = pointer;
        // The terminal's power button (`007ffba0` → `007ffd50`): the
        // menu leaves and the terminal goes at once.
        if power_button {
            use ui::menu::MenuCode;
            power_off_menu = Some(menu);
            match top {
                OpenMenu::Hacking(h) => {
                    h.menu
                        .special_key(ui, ui::menus::hacking::LEAVE, now * 1000.0);
                }
                OpenMenu::Computers(c) => c.menu.close(ui),
                _ => {}
            }
            input.rendered.power_off = true;
        }
        if let (Some((x, y)), true) = (pointer, shown) {
            if let OpenMenu::Vigor(v) = top {
                v.pointer = Some(Vec2::new(x / k, y / k));
            }
            interface.pointer(
                ui,
                menu,
                top.code(),
                x,
                y,
                input.mouse.pressed(MouseButton::Left) || press,
                input.mouse.just_pressed(MouseButton::Left) || press,
                input.mouse.just_released(MouseButton::Left) || release,
                now,
            );
        }
        if let OpenMenu::Vigor(v) = top {
            sounds.0.extend(v.inputs(&game.0, &mut state.0));
        }
        // A meter's bar held under the pointer follows it (`007cf6a0`, id
        // 0x66).
        if let (OpenMenu::Start(s), Some((x, _)), true) = (
            &mut *top,
            pointer,
            shown && input.mouse.pressed(MouseButton::Left),
        ) {
            let bar = interface.dragging.or(interface.over).filter(|&o| {
                ui.has(o, t::ID) && ui.number(o, t::ID) as i32 == ui::menus::start::id::BAR
            });
            if let Some(bar) = bar {
                s.menu.drag_meter(ui, bar, x);
            }
        }
        let notches = match input.scroll.unit {
            MouseScrollUnit::Line => input.scroll.delta.y,
            MouseScrollUnit::Pixel => input.scroll.delta.y / 40.0,
        }
        .round() as i32;
        if shown {
            interface.wheel(ui, menu, top.code(), notches);
        }
        // Keys.
        let shift =
            input.keys.pressed(KeyCode::ShiftLeft) || input.keys.pressed(KeyCode::ShiftRight);
        let alt = input.keys.pressed(KeyCode::AltLeft) || input.keys.pressed(KeyCode::AltRight);
        let events: Vec<KeyboardInput> = input.typed.read().cloned().collect();
        for e in events {
            if e.key_code == KeyCode::KeyT {
                match e.state {
                    ButtonState::Pressed => {
                        rest_pressed |= !e.repeat;
                        *rest_down = true;
                    }
                    ButtonState::Released => *rest_down = false,
                }
            }
            if e.state != ButtonState::Pressed {
                continue;
            }
            // LoveTesterMenu polls scan code1e (A), or10 (Q) in French,
            // for Done (00791ab0). This is separate from its XML bindings.
            if let OpenMenu::Vigor(v) = top {
                let french = game
                    .0
                    .settings
                    .get("General", "sLanguage")
                    .is_some_and(|s| s.eq_ignore_ascii_case("FRENCH"));
                if e.key_code == if french { KeyCode::KeyQ } else { KeyCode::KeyA } {
                    v.menu
                        .intents
                        .push(ui::menus::vigor::InputIntent::TileClick(4));
                    sounds.0.extend(v.inputs(&game.0, &mut state.0));
                    continue;
                }
            }
            // Tab or Escape leave the hacking and terminal menus, the
            // companion wheel and the tutorial box (their code 10).
            if matches!(
                top,
                OpenMenu::Hacking(_)
                    | OpenMenu::Computers(_)
                    | OpenMenu::CompanionWheel(_)
                    | OpenMenu::Tutorial(_)
            ) && matches!(e.logical_key, Key::Escape | Key::Tab)
            {
                use ui::menu::MenuCode;
                match top {
                    OpenMenu::CompanionWheel(w) => {
                        w.menu
                            .special_key(ui, ui::menus::companion_wheel::LEAVE, now * 1000.0);
                    }
                    OpenMenu::Hacking(h) => {
                        h.menu
                            .special_key(ui, ui::menus::hacking::LEAVE, now * 1000.0);
                    }
                    OpenMenu::Computers(c) => {
                        c.menu
                            .special_key(ui, ui::menus::computers::LEAVE, now * 1000.0);
                    }
                    OpenMenu::Tutorial(t) => {
                        t.special_key(ui, ui::menus::tutorial::CLOSE_CODE, now * 1000.0);
                    }
                    _ => {}
                }
                continue;
            }
            // The Escape control: the message box hears it as a 1 (`0070c4a0`).
            if e.logical_key == Key::Escape {
                // The start menu takes the Escape control as Back
                // (`007cf5e0`).
                if let OpenMenu::Start(s) = top {
                    s.menu.escape(ui, now);
                    continue;
                }
                // The recipe menu's Exit is its X key.
                if top.code().class() == ui::menus::recipe::CLASS {
                    interface.key(ui, menu, top.code(), u32::from(b'X'), false, false, now);
                }
                if top.code().class() == ui::menus::message::CLASS {
                    interface.key(
                        ui,
                        menu,
                        top.code(),
                        ui::menu::key::ESCAPE_TO_MESSAGE,
                        false,
                        false,
                        now,
                    );
                }
                continue;
            }
            if let Some(code) = key_code(&e.logical_key) {
                if let OpenMenu::Vigor(v) = top {
                    let (used, played) = v.key(code, &game.0, &mut state.0);
                    sounds.0.extend(played);
                    if used {
                        continue;
                    }
                }
                interface.key(ui, menu, top.code(), code, shift, alt, now);
                if let OpenMenu::Vigor(v) = top {
                    sounds.0.extend(v.inputs(&game.0, &mut state.0));
                }
            }
        }
        // `--menu-keys`: the ones due (`mDX/DY` moves the mouse).
        while let Some(i) = input.fixed_keys.0.iter().position(|(t, _)| *t <= now) {
            let (_, k) = input.fixed_keys.0.remove(i);
            if let Some((dx, dy)) = k
                .strip_prefix('m')
                .and_then(|m| m.split_once('/'))
                .and_then(|(x, y)| Some((x.parse::<f32>().ok()?, y.parse::<f32>().ok()?)))
            {
                println!("--menu-keys: the mouse moved {dx}, {dy}");
                mouse_move.0 += dx;
                mouse_move.1 += dy;
                continue;
            }
            // "tab": what Tab and Escape do in the hacking and terminal
            // menus (their code 10, leaving).
            if k.eq_ignore_ascii_case("tab") {
                use ui::menu::MenuCode;
                println!("--menu-keys: {k}");
                match top {
                    OpenMenu::Hacking(h) => {
                        h.menu
                            .special_key(ui, ui::menus::hacking::LEAVE, now * 1000.0);
                    }
                    OpenMenu::Computers(c) => {
                        c.menu
                            .special_key(ui, ui::menus::computers::LEAVE, now * 1000.0);
                    }
                    _ => {}
                }
                continue;
            }
            let code = match k.to_ascii_lowercase().as_str() {
                "left" => Some(ui::menu::key::LEFT),
                "right" => Some(ui::menu::key::RIGHT),
                "up" => Some(ui::menu::key::UP),
                "down" => Some(ui::menu::key::DOWN),
                "enter" => Some(ui::menu::key::ENTER),
                _ => k.chars().next().map(|c| c as u32),
            };
            if let Some(code) = code {
                println!("--menu-keys: {k}");
                interface.key(ui, menu, top.code(), code, false, false, now);
            }
        }
        // The menus have the keys and buttons.
        input.keys.reset_all();
        // Sounds the interface asked for.
        let order = &game.0.order;
        for effect in interface.effects.drain(..) {
            match effect {
                Effect::Sound(name) => {
                    if let Some(id) = order.form_by_editor_id(&name) {
                        sounds.0.push(id);
                    }
                }
            }
        }
    }
    // The hacking and terminal menus' frames (`00767c90`, `00758470`),
    // after the pointer.
    hacking::update(screen, &state.0, now * 1000.0);
    computers::update(screen, &state.0, now * 1000.0);
    // The sleep/wait menu's frame: its clicks carried out, the hours pass
    // (`007c0580`).
    let rest_held = screen.rest_down;
    sounds.0.extend(sleepwait::frame(
        screen,
        &game.0,
        &mut state.0,
        dt,
        rest_pressed,
        rest_held,
    ));
    // What the menus did.
    let order = &game.0.order;
    message::after(&mut screen.open, &mut state.0, &mut sounds, order);
    sounds.0.extend(container::after(
        screen,
        &game.0,
        &scripts.0,
        &mut state.0,
        &mut wheel_voices,
        library.as_deref_mut(),
        now * 1000.0,
    ));
    sounds
        .0
        .extend(barter::after(screen, &game.0, &mut state.0));
    hud_messages.queue.extend(
        recipe::after(screen, &game.0, &mut state.0)
            .into_iter()
            .map(Into::into),
    );
    sounds
        .0
        .extend(repair::after(screen, &game.0, &mut state.0));
    sounds.0.extend(companion_wheel::after(
        screen,
        &game.0,
        &scripts.0,
        &mut state.0,
        &mut wheel_voices,
        now * 1000.0,
    ));
    sounds.0.extend(caravan::after(
        screen,
        &game.0,
        &mut state.0,
        now * 1000.0,
        caravan_input,
    ));
    for (played, said) in [
        slots::after(
            screen,
            &game.0,
            &mut state.0,
            &mut casino_lock.0,
            now * 1000.0,
        ),
        blackjack::after(
            screen,
            &game.0,
            &mut state.0,
            &mut casino_lock.0,
            now * 1000.0,
        ),
        roulette::after(
            screen,
            &game.0,
            &mut state.0,
            &mut casino_lock.0,
            now * 1000.0,
        ),
    ] {
        sounds.0.extend(played);
        for m in said {
            println!("{}", m.text);
            hud_messages.queue.push(m);
        }
    }
    sounds
        .0
        .extend(levelup::after(screen, &game.0, &mut state.0));
    sounds
        .0
        .extend(traits::after(screen, &game.0, &mut state.0));
    sounds
        .0
        .extend(chargen::after(screen, &game.0, &mut state.0));
    textedit::after(screen, &mut state.0);
    asks::after(screen, &mut asks);
    for m in hacking::after(
        screen,
        order,
        &mut state.0,
        &mut hacking_sounds,
        now * 1000.0,
    ) {
        queue.push(m);
    }
    for n in computers::after(
        screen,
        order,
        &scripts.0,
        &mut state.0,
        &mut hacking_sounds,
        now * 1000.0,
    ) {
        println!("{n}");
        hud_messages.queue.push(n.into());
    }
    // Closed menus leave the screen, fading out (`ui::fade`); a service
    // menu closing brings the conversation back (`007640a0`).
    let mut i = 0;
    while i < screen.open.len() {
        if screen.open[i].closed() {
            let m = screen.open.remove(i);
            let service = m.is_service();
            let how = if power_off_menu == Some(m.tile()) {
                Close::Now
            } else if matches!(m, OpenMenu::Message(_) | OpenMenu::Quantity(_))
                && (queue.pipboy || rendered_up)
            {
                // `007a8df0`, `007abdf0`: at once over a rendered menu.
                Close::Instant
            } else {
                Close::Fade
            };
            screen.close(m, how, dt);
            if screen.open.is_empty() {
                screen.interface.over = None;
                screen.interface.focus = None;
            }
            if service {
                dialog::service_closed(screen, dt);
            }
        } else {
            i += 1;
        }
    }
    // The frame's end: the fades laid on (`00711ea0`).
    screen.end_fades();
    queue.game_open = screen.busy();
    // (A box over the Pip-Boy closing leaves the Pip-Boy up.)
    if !screen.busy() && conversation.0.is_none() && !queue.pipboy {
        player.ready = true;
    }
}

/// The Escape control opens the start menu as the pause menu
/// (`0070c4a0` at `0070e651` → `007cb7d0(1, 0)`) when no other menu has
/// the keys (the game's menus closed, no conversation, no lock; the
/// Pip-Boy may be up: the start menu opens over it). The game stops
/// (virtual time paused) while it's up. Saving is offered unless the game
/// refuses (`00850fe0`: its refusals in combat and the like aren't
/// followed here).
#[allow(clippy::too_many_arguments)]
fn escape_opens_start_menu(
    game: Res<GameFiles>,
    mut menus: ResMut<GameMenus>,
    mut queue: ResMut<crate::menus::Menus>,
    keys: Res<ButtonInput<KeyCode>>,
    conversation: Res<crate::dialogue::Conversation>,
    mut settings: ResMut<start::GameSettings>,
    mut virt: ResMut<Time<Virtual>>,
    mut player: ResMut<crate::walk::Player>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if !keys.just_pressed(KeyCode::Escape) {
        return;
    }
    let Some(size) = window_size(&windows) else {
        return;
    };
    if queue.others_open() || conversation.0.is_some() {
        return;
    }
    let Some(screen) = screen(&mut menus, &game.0, size) else {
        return;
    };
    if !screen.open.is_empty() {
        return;
    }
    let was_paused = virt.is_paused();
    if start::open(screen, &game.0, &mut settings, true, was_paused) {
        virt.pause();
        queue.game_open = true;
        player.ready = false;
    }
}

/// The start menu's frame: its fades and saving, then what it asked for:
/// back to the game (time going again), a save or load for the save
/// system, the program's end, the settings' new values (the volumes into
/// the music's decks).
#[allow(clippy::too_many_arguments)]
pub(crate) fn start_menu_frame(
    game: Res<GameFiles>,
    mut menus: ResMut<GameMenus>,
    mut settings: ResMut<start::GameSettings>,
    mut saves: ResMut<start::SaveFiles>,
    mut virt: ResMut<Time<Virtual>>,
    time: Res<Time<bevy::time::Real>>,
    mut exit: EventWriter<AppExit>,
    mut music: ResMut<crate::music::Music>,
    mut sounds: ResMut<crate::sounds::SoundRequests>,
    state: Res<crate::dialogue::DialogueState>,
) {
    let Some(screen) = menus.screen.as_deref_mut() else {
        return;
    };
    let out = start::frame(screen, &game.0, &mut settings, time.elapsed_secs_f64());
    if out.help {
        tutorial::open_manual(screen, &game.0, &state.0);
    }
    for s in out.sounds {
        if let Some(id) = game.0.order.form_by_editor_id(&s) {
            sounds.0.push(id);
        }
    }
    if let Some(was_paused) = out.resume {
        if !was_paused {
            virt.unpause();
        }
    }
    if let Some(s) = out.save {
        saves.0 = Some(s);
    }
    for (setting, v) in out.applied {
        use ui::menus::start::Setting;
        match setting {
            Setting::MasterVolume => crate::music::set_volumes(&mut music, |vol| vol.master = v),
            Setting::MusicVolume => crate::music::set_volumes(&mut music, |vol| vol.music = v),
            Setting::RadioVolume => crate::music::set_volumes(&mut music, |vol| vol.radio = v),
            _ => {}
        }
    }
    if out.exit {
        println!("Exit Game.");
        exit.write(AppExit::Success);
    }
}

/// The open menus' pictures and the cursor, for the HUD's layer.
#[allow(clippy::too_many_arguments)]
pub(crate) fn draw_menus(
    game: Res<GameFiles>,
    mut menus: ResMut<GameMenus>,
    mut draw: ResMut<MenuDraw>,
    windows: Query<&Window, With<PrimaryWindow>>,
    fixed: Res<FixedPointer>,
    time: Res<Time<bevy::time::Real>>,
    pipboy: Option<Res<crate::pipboy::Pipboy>>,
    mut rendered: ResMut<crate::rendered_terminal::RenderedTerminal>,
    hud: Option<Res<crate::hud::HudLayer>>,
) {
    let rendered_on = rendered.prepare(&game.0, hud.is_some());
    let mut terminal_items: Vec<DrawItem> = Vec::new();
    let terminal_open = menus.screen.as_ref().is_some_and(|s| {
        s.open
            .iter()
            .any(|m| matches!(m, OpenMenu::Hacking(_) | OpenMenu::Computers(_)))
    });
    rendered.menu_open = terminal_open;
    let Some(screen) = menus.screen.as_deref_mut() else {
        rendered.items.clear();
        if !draw.0.is_empty() {
            draw.0.clear();
        }
        if !draw.1.is_empty() {
            draw.1.clear();
        }
        return;
    };
    let mut items: Vec<DrawItem> = Vec::new();
    // The fade to black moves every frame (`007011d0`) and is drawn under
    // the menus while it lasts.
    if let Some(fade) = screen.fade.as_mut() {
        fade.frame(time.delta_secs());
        if !fade.on {
            screen.fade = None;
        }
    }
    {
        let mut files = Files {
            game: &game.0,
            sizes: &mut screen.sizes,
            atlases: &mut screen.atlases,
        };
        if let Some(fader) = screen.fader {
            let alpha = screen.fade.map_or(0.0, |f| f.alpha);
            screen.ui.set_number(fader, t::ALPHA, alpha * 255.0);
            screen
                .ui
                .set_number(fader, t::VISIBLE, f32::from(alpha > 0.0));
            if alpha > 0.0 {
                items.extend(ui::draw_list(&mut screen.ui, fader, &mut files, &|_| None));
            }
        }
        // Menus fading out under the open ones, each with its fade
        // (`ui::fade`).
        for (fading, m) in screen
            .fading
            .iter()
            .map(|m| (true, m))
            .chain(screen.open.iter().map(|m| (false, m)))
        {
            let menu = m.tile();
            // Every picture's own size (`filewidth` / `fileheight`), as the
            // game's tile refresh sets it for any picture: the tutorial box's
            // Vault-Tec symbol and the start menu's title are sized from it.
            ui::draw::update_file_sizes(&mut screen.ui, menu, &mut files);
            let mut list = ui::draw_list(&mut screen.ui, menu, &mut files, &|_| None);
            // The terminal's and the hacking menu's on the terminal's own
            // screen (`rendered_terminal`).
            if rendered_on && matches!(m, OpenMenu::Hacking(_) | OpenMenu::Computers(_)) {
                for item in &list {
                    if let DrawKind::Text { font, .. } = item.kind {
                        if let Some(Some(f)) = screen.ui.fonts.get(font.wrapping_sub(1)) {
                            rendered
                                .font_paths
                                .entry(font)
                                .or_insert_with(|| ui::draw::font_textures(f));
                        }
                    }
                }
                let value = screen.fades.value(menu);
                ui::fade::fade_items(&mut screen.ui, menu, value, &mut list);
                terminal_items.extend(list);
            } else {
                let first = items.len();
                items.extend(list);
                // A start menu's `nif` tile (the pause background).
                start::background_draws(
                    &mut screen.ui,
                    if fading { &screen.fading } else { &screen.open },
                    &game.0,
                    &mut screen.models,
                    &mut items,
                    menu,
                    first,
                );
                let value = screen.fades.value(menu);
                ui::fade::fade_items(&mut screen.ui, menu, value, &mut items[first..]);
            }
        }
        // The cursor over everything while a menu is open (the Pip-Boy
        // too: it moves over the screen and the model's buttons,
        // `007f8720`, except where DATA's map hides it, `0079a130`) and
        // the pointer is in the window.
        let wanted = !screen.open.is_empty()
            || !screen.fading.is_empty()
            || pipboy
                .as_ref()
                .is_some_and(|p| p.open && !p.cursor_hidden());
        if let Some(cursor) = screen.cursor {
            let k = screen.ui.screen_size.resolution_converter();
            let at = match fixed.0 {
                Some((x, y)) => Some(Vec2::new(x, y) * (960.0 / 1080.0) / k),
                None => windows
                    .single()
                    .ok()
                    .and_then(|w| w.physical_cursor_position()),
            };
            // Roulette hides the pointer while it's on top (`007bd2a0`).
            let hidden = matches!(screen.open.last(), Some(OpenMenu::Roulette(_)));
            match (at, wanted && !hidden) {
                (Some(p), true) => {
                    screen.ui.set_number(cursor, t::X, p.x * k);
                    screen.ui.set_number(cursor, t::Y, p.y * k);
                    screen.ui.set_number(cursor, t::VISIBLE, 1.0);
                    items.extend(ui::draw_list(&mut screen.ui, cursor, &mut files, &|_| None));
                }
                _ => screen.ui.set_number(cursor, t::VISIBLE, 0.0),
            }
        }
    }
    let _ = kind::IMAGE;
    if rendered.items != terminal_items {
        rendered.items = terminal_items;
    }
    if draw.0 != items {
        draw.0 = items;
    }
    // The Pip-Boy's menu first while it's up: the menus over it opened
    // from it, and the HUD keeps the pieces the Pip-Boy's pages leave on
    // (`ui::hud::parts_for_menu`: the messages).
    let classes: Vec<i32> = pipboy
        .as_ref()
        .and_then(|p| p.menu_class())
        .into_iter()
        .chain(screen.open.iter_mut().map(|m| m.code().class()))
        .collect();
    if draw.1 != classes {
        draw.1 = classes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ui::fade::State;
    use ui::menus::message::MessageMenu;

    /// The menus with no game files: a 1920 × 1080 screen.
    fn bare() -> Screen {
        let ui = ui::Ui::new(
            ui::Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            ui::SystemColors::new(None, None),
            Box::new(|_| None),
        );
        Screen {
            ui,
            interface: Interface::default(),
            open: Vec::new(),
            fading: Vec::new(),
            fades: ui::fade::Fades::default(),
            size: UVec2::new(1920, 1080),
            cursor: None,
            fade: None,
            fader: None,
            rest_down: false,
            quantity_owner: None,
            pointer: None,
            mouse_move: (0.0, 0.0),
            terminal_left: false,
            sizes: HashMap::new(),
            atlases: HashMap::new(),
            models: HashMap::new(),
        }
    }

    fn open_message(s: &mut Screen) -> TileId {
        let tile =
            s.ui.load_menu(
                b"<menu name=\"M\"><image name=\"a\"><filename>solid.dds</filename></image></menu>",
                &mut |_| None,
            )
            .unwrap();
        s.ui.set_number(tile, t::VISIBLE, 1.0);
        s.open.push(OpenMenu::Message(MessageMenu::new(tile)));
        tile
    }

    /// A menu opened fades in over its `menufade` (0.25), the pointer
    /// reaching it only once it's shown; closed, it fades out, drawn but
    /// not run, holding the game until it's gone.
    #[test]
    fn menus_fade_in_and_out() {
        let mut s = bare();
        let m = open_message(&mut s);
        s.show_new(0.1);
        s.end_fades();
        assert_eq!(s.fades.state(m), State::FadingIn);
        assert!((s.fades.value(m) - 0.4).abs() < 1e-6);
        assert!(!s.fades.takes_pointer(m));
        s.fades.update(0.2);
        s.end_fades();
        assert_eq!(s.fades.state(m), State::Shown);
        assert!(s.fades.takes_pointer(m));
        // Closed.
        s.ui.set_number(m, ui::menu::LEAVE_STACK, 1.0);
        let closed = s.open.remove(0);
        s.close(closed, Close::Fade, 0.1);
        assert!(s.open.is_empty() && s.busy());
        s.end_fades();
        assert!((s.fades.value(m) - 0.6).abs() < 1e-6);
        assert!(s.ui.is_under(m, s.ui.screen));
        s.fades.update(0.2);
        s.end_fades();
        assert!(!s.busy());
        assert!(!s.ui.is_under(m, s.ui.screen));
    }

    /// The power button's and an instant close.
    #[test]
    fn menus_closed_at_once() {
        let mut s = bare();
        let m = open_message(&mut s);
        s.show_new(1.0);
        s.end_fades();
        let closed = s.open.remove(0);
        s.close(closed, Close::Now, 0.0);
        assert!(!s.busy() && !s.ui.is_under(m, s.ui.screen));
        let m = open_message(&mut s);
        s.show_new(1.0);
        s.end_fades();
        s.ui.set_number(m, ui::menu::LEAVE_STACK, 1.0);
        let closed = s.open.remove(0);
        s.close(closed, Close::Instant, 0.0);
        assert_eq!(s.ui.number(m, t::VISIBLE), 0.0);
        s.end_fades();
        assert!(!s.busy());
    }

    /// `--answer-boxes`: a box with one button gets it; one with more the
    /// next choice given, in turn; with none left it's left open.
    #[test]
    fn answered_boxes_take_their_choices_in_turn() {
        let mut choices = std::collections::VecDeque::from(vec![1, 0]);
        assert_eq!(box_choice(1, &mut choices), Some(0));
        assert_eq!(choices.len(), 2);
        assert_eq!(box_choice(2, &mut choices), Some(1));
        assert_eq!(box_choice(3, &mut choices), Some(0));
        assert_eq!(box_choice(2, &mut choices), None);
        assert_eq!(box_choice(0, &mut choices), None);
    }

    /// A service menu over the conversation (`00763ff0`, `007640a0`): the
    /// dialogue menu fades out and back in on the same fades as every
    /// menu's, kept open (not marked to leave the stack) and hidden
    /// meanwhile, then shown again.
    #[test]
    fn the_conversation_fades_under_a_service_menu_and_back() {
        let mut s = bare();
        let tile =
            s.ui.load_menu(
                b"<menu name=\"DialogMenu\"><image name=\"a\"><filename>solid.dds</filename></image></menu>",
                &mut |_| None,
            )
            .unwrap();
        s.open
            .push(OpenMenu::Dialog(ui::menus::dialog::DialogMenu::new(tile)));
        s.show_new(1.0);
        s.end_fades();
        assert_eq!(s.fades.state(tile), State::Shown);
        dialog::service_opened(&mut s, 0.0);
        assert!(dialog::hidden(&mut s));
        assert_eq!(s.fades.state(tile), State::FadingOut);
        s.fades.update(1.0);
        s.end_fades();
        assert_eq!(s.fades.state(tile), State::Hidden);
        assert_eq!(s.ui.number(tile, t::VISIBLE), 0.0);
        assert_eq!(s.open.len(), 1);
        assert!(s.ui.is_under(tile, s.ui.screen));
        dialog::service_closed(&mut s, 0.0);
        assert!(!dialog::hidden(&mut s));
        assert_eq!(s.fades.state(tile), State::FadingIn);
        s.fades.update(1.0);
        s.end_fades();
        assert_eq!(s.fades.state(tile), State::Shown);
        assert_eq!(s.fades.value(tile), 1.0);
        assert_eq!(s.ui.number(tile, t::VISIBLE), 1.0);
    }
}

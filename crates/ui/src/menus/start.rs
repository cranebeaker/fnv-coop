//! The start menu (`menus\options\start_menu.xml`, class `StartMenu` 1013,
//! vtable `01076d1c` in FalloutNV.exe 1.4.0.525; `StartMenu` (Xbox PDB)).
//! The Escape control opens it over the game as the pause menu
//! (`0070c4a0` → `007cb7d0(1, 0)`); its code:
//!
//! * its options (`007cc6e0`): two arrays, the main options (011dab88:
//!   Continue, New, Save, Load, Settings, Help, Credits, Downloads, Quit,
//!   the four settings pages, the confirmations' Yes / No, Main Menu, Exit
//!   Game, Cancel, Action Mapping) and the user options (011dab50: the
//!   settings pages' toggles and meters), each with a label (a text
//!   setting), what it does and the pages it's on (a mask: 1 the main
//!   menu, 2 the pause menu, 4 the settings list, 8 Gameplay, 0x10
//!   Display, 0x20 Audio, 0x40 Controls, 0x80 .. 0x2000 the
//!   confirmations);
//! * opening (`007cbaf0`): the pause menu's options (mask 2) in the main
//!   list, the settings pages (mask 4) in the settings list, an option the
//!   game can't do now disabled (`_enabled` 0: Save when `00850fe0` says
//!   no, Load without saves), the texts, `main_version` "1.4.0.525",
//!   `user0` (16:9 or wider), the main list faded in and enabled, every
//!   user option reading its setting (`007d3aa0`), the background model
//!   scaled by 1.3333 (`01073f20`);
//! * pages fade (`007d61e0`): a container's `_alpha` (or the `alpha` the
//!   caller names) from 0 to 255 or back over `0101622c` 0.25 s when the
//!   given time is below 0 (the callers pass `01012054` −1), straight
//!   (`00a07c60` mode 0); a container is visible while its `_alpha` is
//!   above 0 (the file);
//! * a click (`007ce9b0`): Back (id 5), Defaults / Delete (id 4), the
//!   settings list itself (id 1: back from a settings page), then, 500 ms
//!   after the last (`011daab4`), the option clicked in the main,
//!   settings or confirmation list does its thing (the list it's in
//!   disabled first unless it's the confirmation's or Back), a save in the
//!   save/load list is chosen, or a user option's value steps (the arrows
//!   100 / 101 or the row);
//! * Back (`007d0e40`), the pages' handlers (`007d0440` .. `007d0c50`,
//!   `007d42b0`, `007d48a0`), the frame (`007cf9a0`: saving, a user
//!   option's change carried out), the arrow keys and pages (`007cefc0`),
//!   the Escape control (`007cf5e0`: Back);
//! * the save list (`007d3b60`, `007d3c70`, `007d67b0`): saves newest
//!   first, "AUTO" / "QUICK" / "#0001" and the place; in save mode a
//!   "[NEW SAVE] n/1000 saves used." line first.
//!
//! The world's side (saving, loading, closing, the settings' values) is
//! the caller's, through [`Request`]s.

use crate::anim::Animations;
use crate::list::ListBox;
use crate::menu::{special, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\options\\start_menu.xml";
/// Its class number (`007cb4d0`).
pub const CLASS: i32 = 1013;
/// A fade's length when the caller gives none (`0101622c`).
pub const FADE_SECONDS: f32 = 0.25;
/// The background model's scale (`01073f20`, `007cbaf0`).
pub const BACKGROUND_SCALE: f32 = 1.3333;
/// Clicks closer together than this are dropped (`007ce9b0`, `007cf5e0`).
pub const CLICK_GAP_MS: f64 = 500.0;
/// How long "Saving..." stays before the menu closes (`007d5af0`: 3000).
pub const SAVE_WAIT_MS: f64 = 3000.0;
/// The game's version on `main_version` (`007cbaf0`).
pub const VERSION: &str = "1.4.0.525";
/// Saves the game counts up to (`007d3c70`: "%s %u/%i %s" with 1000).
pub const MAX_SAVES: usize = 1000;

/// The file's tiles by `id` (`007cb4e0` keeps ids below 0x17).
pub mod id {
    pub const MAIN: i32 = 0;
    pub const SETTINGS: i32 = 1;
    pub const OPTIONS: i32 = 2;
    pub const TITLE: i32 = 3;
    pub const DEFAULTS: i32 = 4;
    pub const BACK: i32 = 5;
    pub const SAVELOAD: i32 = 6;
    pub const CONFIRM: i32 = 7;
    pub const QUESTION: i32 = 8;
    pub const PRESS_START: i32 = 9;
    pub const VERSION: i32 = 10;
    pub const SCREENSHOT: i32 = 11;
    pub const PLAYER_NAME: i32 = 12;
    pub const BACKGROUND: i32 = 13;
    pub const DOWNLOADS: i32 = 14;
    pub const DEVICE: i32 = 15;
    pub const CONTROLS_HEADER: i32 = 16;
    pub const PLAYER_LEVEL: i32 = 18;
    pub const PLAY_TIME: i32 = 19;
    pub const WARNING: i32 = 20;
    pub const SAVE_VERSION: i32 = 22;
    /// A meter's or toggle's arrows and a meter's bar.
    pub const LEFT: i32 = 100;
    pub const RIGHT: i32 = 101;
    pub const BAR: i32 = 102;
}

/// The menu's flags (`+0x1a8`, `0075f6f0` / `004a4080`).
pub mod flag {
    pub const PAUSE: u32 = 1;
    pub const SETTINGS_CHANGED: u32 = 2;
    pub const SAVE_MODE: u32 = 8;
    pub const APPLY: u32 = 0x20;
    pub const DELETE_MODE: u32 = 0x2000;
    pub const SAVE_ASKED: u32 = 0x4000;
    pub const SAVING: u32 = 0x8000;
    pub const BACK_TO_SAVES: u32 = 0x40000;
    pub const SAVED: u32 = 0x1000_0000;
}

/// The pages an option is on.
pub mod page {
    pub const TITLE: u32 = 1;
    pub const PAUSE: u32 = 2;
    pub const SETTINGS: u32 = 4;
    pub const GAMEPLAY: u32 = 8;
    pub const DISPLAY: u32 = 0x10;
    pub const AUDIO: u32 = 0x20;
    pub const CONTROLS: u32 = 0x40;
    pub const CONFIRM_NEW: u32 = 0x80;
    pub const CONFIRM_CONTINUE: u32 = 0x100;
    pub const CONFIRM_LOAD: u32 = 0x200;
    pub const CONFIRM_SAVE: u32 = 0x400;
    pub const CONFIRM_DELETE: u32 = 0x800;
    pub const QUIT_TITLE: u32 = 0x1000;
    pub const QUIT_PAUSE: u32 = 0x2000;
}

/// What a main option does (its handler).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// `007d0440`: in the pause menu, back to the game (`007ce7a0(1)`);
    /// on the main menu, "Continue from your last saved game?".
    Continue,
    /// `007d0490`: "Start a new game?" (`sConfirmNew`).
    New,
    /// `007d06c0` / `007d0680`: the save list in save or load mode.
    Save,
    Load,
    /// `007d0700`: the settings list.
    Settings,
    /// `007d0770`: the help (tutorial) menu for the menu under it.
    Help,
    /// `007d04d0`, `007d0550`.
    Credits,
    Downloads,
    /// `007d0a10`: "Are you sure you want to quit?" with Main Menu / Exit
    /// Game / Cancel in the pause menu, Exit Game / Cancel on the main one.
    Quit,
    /// `007d0d40` .. `007d0df0`: a settings page (its mask).
    Page(u32),
    /// The confirmations' Yes: `007d32b0` new game, `007d3190` continue,
    /// `007d3470` load, `007d35c0` save over, `007d3620` delete.
    YesNew,
    YesContinue,
    YesLoad,
    YesSave,
    YesDelete,
    /// `007d0e40` (No, Cancel and Back).
    Back,
    /// `007d0a70`: to the main menu.
    MainMenu,
    /// `007d0bf0`: quits the game.
    ExitGame,
    /// `007d0c50`: the key bindings page.
    ActionMapping,
}

/// A user option's setting (`StartMenu::Set…` (Xbox PDB)).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Setting {
    KillCam,
    Hardcore,
    Difficulty,
    SaveOnRest,
    SaveOnWait,
    SaveOnTravel,
    TrueIronSights,
    Brightness,
    HudOpacity,
    HudColor,
    PipboyColor,
    TextureSize,
    ActorFade,
    ItemFade,
    ObjectFade,
    GrassFade,
    ShadowFade,
    LightFade,
    SpecularFade,
    ObjectLodFade,
    TreeLodFade,
    Crosshair,
    DialogueSubtitles,
    GeneralSubtitles,
    MasterVolume,
    MusicVolume,
    FootstepVolume,
    VoiceVolume,
    EffectsVolume,
    RadioVolume,
    InvertY,
    MouseSensitivity,
    Controller,
    Rumble,
}

/// How a user option shows its value (`007d6350`'s type, `+0x1c`).
#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    /// A main option: a line of text.
    Plain(Action),
    /// Type 0 (values wrap round) or 1 (they stop at the ends): the value's
    /// text between arrows `spacing` (`user4`) apart; `lb_toggle_template`.
    Toggle {
        setting: Setting,
        values: Vec<String>,
        wrap: bool,
        spacing: f32,
    },
    /// Type 2: `ticks` steps (`+0x20`) on a meter; `lb_meter_template`.
    Meter { setting: Setting, ticks: i32 },
}

/// One option (`StartMenu::Option` / `UserOption` (Xbox PDB)).
#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub label: String,
    /// The pages it's on (`+0xc`).
    pub pages: u32,
    pub kind: Kind,
    /// The value shown (`+0x14`) and the default (`+0x18`).
    pub value: i32,
    pub default: i32,
}

impl Item {
    fn count(&self) -> i32 {
        match &self.kind {
            Kind::Plain(_) => 0,
            Kind::Toggle { values, .. } => values.len() as i32,
            Kind::Meter { ticks, .. } => *ticks,
        }
    }

    pub fn setting(&self) -> Option<Setting> {
        match &self.kind {
            Kind::Toggle { setting, .. } | Kind::Meter { setting, .. } => Some(*setting),
            Kind::Plain(_) => None,
        }
    }

    fn template(&self) -> &'static str {
        match &self.kind {
            Kind::Plain(_) => "lb_item_template",
            Kind::Toggle { .. } => "lb_toggle_template",
            Kind::Meter { .. } => "lb_meter_template",
        }
    }
}

/// A save as the list shows it.
#[derive(Debug, Clone, PartialEq)]
pub struct SaveEntry {
    /// What the caller knows it by.
    pub key: String,
    /// "AUTO", "QUICK" or "#0001" (`007d67b0`).
    pub prefix: String,
    /// Where it was made (`+0x14`), or `sSaveGameCorrupt`.
    pub place: String,
    pub player: String,
    pub level: Option<i32>,
    /// Play time as the game shows it.
    pub play_time: String,
    pub corrupt: bool,
}

/// What the menu asks the game to do.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Back to the game (`007ce7a0(1)`): the menu closes.
    Resume,
    /// A new save (`008503b0`) or one written over (`00846710`).
    SaveNew,
    SaveOver(String),
    /// A save loaded (`008467b0`); the menu closes when it worked.
    Load(String),
    /// A save deleted (`008467e0`).
    Delete(String),
    /// `007d3190`: the newest save loaded.
    LoadNewest,
    /// `007d0a70`: the game torn down for the main menu.
    MainMenu,
    /// `007d0bf0`: the program ends.
    ExitGame,
    /// `007d0770`: the help for the menu under the start menu.
    Help,
    /// A user option changed (`flag::APPLY` set: its handler writes the
    /// setting): the setting and its new value index of `count`.
    Apply {
        setting: Setting,
        value: i32,
        count: i32,
    },
    /// Settings changed and the menu went back: the preferences written
    /// (`007d6d70`).
    SavePreferences,
}

/// Which list a row is in and what it is.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Row {
    Main(usize),
    User(usize),
    Save(Option<usize>),
}

/// The start menu.
pub struct StartMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; 23],
    pub flags: u32,
    pub main_items: Vec<Item>,
    pub user_items: Vec<Item>,
    /// The lists (`+0x84` main, `+0xb4` settings, `+0xe4` confirmation,
    /// `+0x114` options, `+0x174` saves).
    pub main: ListBox,
    pub settings: ListBox,
    pub confirm: ListBox,
    pub options: ListBox,
    pub saves: ListBox,
    rows: Vec<(TileId, Row)>,
    pub save_entries: Vec<SaveEntry>,
    /// The save a click chose (`+0x1b0`).
    chosen_save: Option<usize>,
    /// A user option whose value changed (`+0x1a4`), carried out next frame.
    changed: Option<usize>,
    /// The last click (`011daab4`) and when "Saving..." started (`011dac68`).
    last_click: f64,
    save_until: f64,
    pub anims: Animations,
    pub now: f64,
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
}

/// The texts a menu needs (`ui.setting_text`).
fn text(ui: &Ui, name: &str) -> String {
    ui.setting_text(name).unwrap_or_default()
}

/// The main options, in `007cc6e0`'s order.
// Translated from 007cc6e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn main_items(ui: &Ui) -> Vec<Item> {
    let plain = |label: &str, pages: u32, action: Action| Item {
        label: text(ui, label),
        pages,
        kind: Kind::Plain(action),
        value: 0,
        default: 0,
    };
    vec![
        plain("sContinue", 3, Action::Continue),
        plain("sNew", 1, Action::New),
        plain("sSave", 2, Action::Save),
        plain("sLoad", 3, Action::Load),
        plain("sSettings", 3, Action::Settings),
        plain("sHelp", 2, Action::Help),
        plain("sCrew", 1, Action::Credits),
        plain("sDownloads", 1, Action::Downloads),
        plain("sQuit", 3, Action::Quit),
        plain("sGameplay", 4, Action::Page(page::GAMEPLAY)),
        plain("sDisplay", 4, Action::Page(page::DISPLAY)),
        plain("sAudio", 4, Action::Page(page::AUDIO)),
        plain("sControls", 4, Action::Page(page::CONTROLS)),
        plain("sYes", page::CONFIRM_NEW, Action::YesNew),
        plain("sYes", page::CONFIRM_CONTINUE, Action::YesContinue),
        plain("sYes", page::CONFIRM_LOAD, Action::YesLoad),
        plain("sYes", page::CONFIRM_SAVE, Action::YesSave),
        plain("sYes", page::CONFIRM_DELETE, Action::YesDelete),
        plain("sNo", 0xf80, Action::Back),
        plain("sMainMenu", page::QUIT_PAUSE, Action::MainMenu),
        plain("sExitGameAffirm", 0x3000, Action::ExitGame),
        plain("sCancel", 0x3000, Action::Back),
        plain("sActionMapping", page::CONTROLS, Action::ActionMapping),
    ]
}

/// The user options, in `007cc6e0`'s order: (label, setting, page, type,
/// count, arrow spacing, value labels).
// Translated from 007cc6e0 (decompiled, FalloutNV.exe 1.4.0.525)
pub fn user_items(ui: &Ui) -> Vec<Item> {
    use Setting::*;
    let on_off = || vec![text(ui, "sOff"), text(ui, "sOn")];
    let toggle = |label: &str, setting, pages, wrap, spacing: f32, values: Vec<String>| Item {
        label: text(ui, label),
        pages,
        kind: Kind::Toggle {
            setting,
            values,
            wrap,
            spacing,
        },
        value: 0,
        default: 0,
    };
    let meter = |label: &str, setting, pages, ticks| Item {
        label: text(ui, label),
        pages,
        kind: Kind::Meter { setting, ticks },
        value: 0,
        default: 0,
    };
    let names = |list: &[&str]| list.iter().map(|n| text(ui, n)).collect::<Vec<_>>();
    vec![
        toggle(
            "sKillCamMode",
            KillCam,
            page::GAMEPLAY,
            true,
            85.0,
            names(&[
                "sKillCamModeNone",
                "sKillCamModePlayer",
                "sKillCamModeCinematic",
            ]),
        ),
        toggle("sHardcore", Hardcore, page::GAMEPLAY, true, 0.0, on_off()),
        toggle(
            "sDifficulty",
            Difficulty,
            page::GAMEPLAY,
            true,
            85.0,
            names(&[
                "sLockLevelNameVeryEasy",
                "sLockLevelNameEasy",
                "sNormal",
                "sLockLevelNameHard",
                "sLockLevelNameVeryHard",
            ]),
        ),
        toggle(
            "sSaveOnRest",
            SaveOnRest,
            page::GAMEPLAY,
            true,
            0.0,
            on_off(),
        ),
        toggle(
            "sSaveOnWait",
            SaveOnWait,
            page::GAMEPLAY,
            true,
            0.0,
            on_off(),
        ),
        toggle(
            "sSaveOnTravel",
            SaveOnTravel,
            page::GAMEPLAY,
            true,
            0.0,
            on_off(),
        ),
        toggle(
            "sTrueIronSights",
            TrueIronSights,
            page::GAMEPLAY,
            true,
            0.0,
            on_off(),
        ),
        meter("sBrightness", Brightness, page::DISPLAY, 16),
        meter("sHUDOpacity", HudOpacity, page::DISPLAY, 16),
        toggle(
            "sHUDColor",
            HudColor,
            page::DISPLAY,
            true,
            55.0,
            names(&["sGreen", "sBlue", "sAmber", "sWhite"]),
        ),
        toggle(
            "sPipboyColor",
            PipboyColor,
            page::DISPLAY,
            true,
            55.0,
            names(&["sGreen", "sBlue", "sAmber", "sWhite"]),
        ),
        toggle(
            "sTextureSize",
            TextureSize,
            page::DISPLAY,
            true,
            55.0,
            names(&["sLarge", "sMedium", "sSmall"]),
        ),
        meter("sActorFade", ActorFade, page::DISPLAY, 16),
        meter("sItemFade", ItemFade, page::DISPLAY, 16),
        meter("sObjectFade", ObjectFade, page::DISPLAY, 16),
        meter("sGrassFade", GrassFade, page::DISPLAY, 16),
        meter("sShadowFade", ShadowFade, page::DISPLAY, 16),
        meter("sLightFade", LightFade, page::DISPLAY, 16),
        meter("sSpecularityFade", SpecularFade, page::DISPLAY, 16),
        meter("sObjectLODFade", ObjectLodFade, page::DISPLAY, 16),
        meter("sTreeLODFade", TreeLodFade, page::DISPLAY, 16),
        toggle("sCrosshair", Crosshair, page::DISPLAY, true, 0.0, on_off()),
        toggle(
            "sDialogSubtitles",
            DialogueSubtitles,
            page::DISPLAY,
            true,
            0.0,
            on_off(),
        ),
        toggle(
            "sGeneralSubtitles",
            GeneralSubtitles,
            page::DISPLAY,
            true,
            0.0,
            on_off(),
        ),
        meter("sMasterVolume", MasterVolume, page::AUDIO, 26),
        meter("sMusicVolume", MusicVolume, page::AUDIO, 26),
        meter("sFootstepsVolume", FootstepVolume, page::AUDIO, 26),
        meter("sVoiceVolume", VoiceVolume, page::AUDIO, 26),
        meter("sEffectsVolume", EffectsVolume, page::AUDIO, 26),
        meter("sRadioVolume", RadioVolume, page::AUDIO, 26),
        toggle("sCameraPitch", InvertY, page::CONTROLS, true, 0.0, on_off()),
        meter("sMouseSensitivity", MouseSensitivity, page::CONTROLS, 16),
        toggle(
            "sDisableXBoxController",
            Controller,
            page::CONTROLS,
            true,
            0.0,
            on_off(),
        ),
        toggle("sRumble", Rumble, page::CONTROLS, true, 0.0, on_off()),
    ]
}

impl StartMenu {
    pub fn new(menu: TileId, ui: &Ui) -> StartMenu {
        StartMenu {
            menu,
            tiles: [None; 23],
            flags: 0,
            main_items: main_items(ui),
            user_items: user_items(ui),
            main: ListBox::default(),
            settings: ListBox::default(),
            confirm: ListBox::default(),
            options: ListBox::default(),
            saves: ListBox::default(),
            rows: Vec::new(),
            save_entries: Vec::new(),
            chosen_save: None,
            changed: None,
            last_click: f64::NEG_INFINITY,
            save_until: 0.0,
            anims: Animations::default(),
            now: 0.0,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: i32) -> Option<TileId> {
        self.tiles.get(id as usize).copied().flatten()
    }

    fn has(&self, f: u32) -> bool {
        self.flags & f != 0
    }

    fn set(&mut self, f: u32, on: bool) {
        if on {
            self.flags |= f;
        } else {
            self.flags &= !f;
        }
    }

    fn custom(ui: &mut Ui, name: &str) -> i32 {
        ui.names.lookup_or_add(name).unwrap_or(0)
    }

    /// `007d61e0`: fades a tile's `_alpha` (or `trait_id`) in (0 → 255)
    /// or out (255 → 0) over `seconds` (below 0: 0.25), unless it's there.
    // Translated from 007d61e0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn fade(&mut self, ui: &mut Ui, tile_id: i32, fade_in: bool, trait_id: Option<i32>) {
        let trait_id = trait_id.unwrap_or_else(|| Self::custom(ui, "_alpha"));
        let Some(tile) = self.tile(tile_id) else {
            return;
        };
        let value = ui.number(tile, trait_id);
        if fade_in && value < 255.0 {
            self.anims
                .start(tile, trait_id, 0.0, 255.0, FADE_SECONDS, self.now);
        } else if !fade_in && value > 0.0 {
            self.anims
                .start(tile, trait_id, 255.0, 0.0, FADE_SECONDS, self.now);
        }
    }

    /// A list's `_enabled` (`007d74f0`, `007d8810`).
    fn enable(ui: &mut Ui, list: &ListBox, on: bool) {
        if let Some(l) = list.list {
            let e = Self::custom(ui, "_enabled");
            ui.set_number(l, e, f32::from(u8::from(on)));
        }
    }

    fn list_enabled(ui: &mut Ui, list: &ListBox) -> bool {
        list.enabled(ui)
    }

    /// `007d7630`: the list's tile is shown.
    fn list_shown(ui: &mut Ui, list: &ListBox) -> bool {
        list.list.is_some_and(|l| ui.shown(l))
    }

    fn clear_list(&mut self, ui: &mut Ui, which: u8) {
        let list = match which {
            0 => &mut self.main,
            1 => &mut self.settings,
            2 => &mut self.confirm,
            3 => &mut self.options,
            _ => &mut self.saves,
        };
        let gone: Vec<TileId> = list.items.iter().map(|i| i.tile).collect();
        list.clear(ui);
        self.rows.retain(|(t, _)| !gone.contains(t));
    }

    /// `007d6ff0`: an option added to a list as a line of its template.
    fn add_row(
        list: &mut ListBox,
        ui: &mut Ui,
        menu: TileId,
        template: &str,
        label: &str,
    ) -> Option<TileId> {
        let keep = std::mem::replace(&mut list.template, template.to_string());
        let tile = list.add(ui, menu, 0, Some(label));
        list.template = keep;
        tile
    }

    /// Opens it (`007cbaf0`): `pause` from the game (the Escape control),
    /// else the main menu; `can_save` (`00850fe0(0)`), whether there are
    /// saves (the `.fos` search), `widescreen` (aspect above 1.3333,
    /// `01076e60`).
    // Translated from 007cbaf0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn open(
        &mut self,
        ui: &mut Ui,
        pause: bool,
        can_save: bool,
        have_saves: bool,
        widescreen: bool,
        settings: &dyn Fn(Setting, i32) -> (i32, i32),
    ) -> bool {
        let (Some(main), Some(set), Some(conf), Some(opts), Some(sl)) = (
            self.tile(id::MAIN),
            self.tile(id::SETTINGS),
            self.tile(id::CONFIRM),
            self.tile(id::OPTIONS),
            self.tile(id::SAVELOAD),
        ) else {
            return false;
        };
        self.set(flag::PAUSE, pause);
        self.main = ListBox::new(ui, main, "lb_item_template");
        self.settings = ListBox::new(ui, set, "lb_item_template");
        self.confirm = ListBox::new(ui, conf, "lb_item_template");
        self.options = ListBox::new(ui, opts, "lb_item_template");
        self.saves = ListBox::new(ui, sl, "lb_saveload_template");
        let mine = if pause { page::PAUSE } else { page::TITLE };
        let menu = self.menu;
        let enabled = Self::custom(ui, "_enabled");
        for i in 0..self.main_items.len() {
            let item = self.main_items[i].clone();
            let Kind::Plain(action) = item.kind else {
                continue;
            };
            let disabled = match action {
                Action::Save => !can_save,
                Action::Continue => !pause && !have_saves,
                Action::Load => !have_saves,
                _ => false,
            };
            let row = if item.pages & mine != 0 {
                Self::add_row(&mut self.main, ui, menu, "lb_item_template", &item.label)
            } else if item.pages & page::SETTINGS != 0 {
                Self::add_row(
                    &mut self.settings,
                    ui,
                    menu,
                    "lb_item_template",
                    &item.label,
                )
            } else {
                None
            };
            if let Some(row) = row {
                self.rows.push((row, Row::Main(i)));
                if disabled {
                    ui.set_number(row, enabled, 0.0);
                }
            }
        }
        // `_number_of_visible_items` of the main and settings lists: their
        // counts (`00715c60`).
        let visible = Self::custom(ui, "_number_of_visible_items");
        let n = self.main.items.len() as f32;
        ui.set_number(main, visible, n);
        let n = self.settings.items.len() as f32;
        ui.set_number(set, visible, n);
        // The texts (`007cbaf0`'s four `0xfc4`s): the version, and the
        // tiles the file leaves to the code.
        for (tile_id, name) in [
            (id::PRESS_START, "sPressStart"),
            (id::WARNING, "sConfirmWarning"),
            (id::DEVICE, "sDevice"),
            (id::BACK, "sBack"),
        ] {
            if let Some(tile) = self.tile(tile_id) {
                ui.set_string(tile, t::STRING, &text(ui, name));
            }
        }
        if let Some(v) = self.tile(id::VERSION) {
            ui.set_string(v, t::STRING, VERSION);
            ui.set_number(v, t::VISIBLE, 0.0);
        }
        ui.set_number(self.menu, t::USER0, f32::from(u8::from(widescreen)));
        if pause {
            // The title off (the pause background shows while it's
            // hidden), the main list in and on.
            if let Some(title) = self.tile(id::TITLE) {
                ui.set_number(title, t::VISIBLE, 0.0);
            }
            self.fade(ui, id::MAIN, true, None);
            Self::enable(ui, &self.main.clone(), true);
        }
        self.read_settings(settings);
        ui.refresh();
        true
    }

    /// `007d3aa0`: every user option reads its setting (`flag::APPLY` off
    /// meanwhile): `settings(setting, count)` gives (value, default).
    pub fn read_settings(&mut self, settings: &dyn Fn(Setting, i32) -> (i32, i32)) {
        self.set(flag::APPLY, false);
        for item in self.user_items.iter_mut() {
            if let Some(s) = item.setting() {
                let (v, d) = settings(s, item.count());
                item.value = v.clamp(0, (item.count() - 1).max(0));
                item.default = d.clamp(0, (item.count() - 1).max(0));
            }
        }
        self.set(flag::APPLY, true);
    }

    /// One frame (`007cf9a0`): the fades, a save under way, a user
    /// option's change carried out.
    // Translated from 007cf9a0 / 007d5af0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn update(&mut self, ui: &mut Ui, now: f64) {
        self.now = now;
        self.anims.step(ui, now);
        if self.has(flag::SAVE_ASKED) {
            self.save_until = now + SAVE_WAIT_MS / 1000.0;
            self.set(flag::SAVE_ASKED, false);
            self.set(flag::SAVING, true);
        } else if self.has(flag::SAVING) {
            if !self.has(flag::SAVED) {
                match self.chosen_save.and_then(|i| self.save_entries.get(i)) {
                    Some(s) => self.requests.push(Request::SaveOver(s.key.clone())),
                    None => self.requests.push(Request::SaveNew),
                }
                self.set(flag::SAVED, true);
            }
            if now >= self.save_until {
                self.resume();
                self.set(flag::SAVING, false);
                self.set(flag::SAVED, false);
            }
        }
        // `007d62e0`: back to the save list: enabled, the line unmarked,
        // every line shown again (`user0` 1).
        if self.has(flag::BACK_TO_SAVES) {
            Self::enable(ui, &self.saves.clone(), true);
            let selected = Self::custom(ui, "_selected");
            for item in self.saves.items.clone() {
                ui.set_number(item.tile, selected, 0.0);
            }
            if let Some(l) = self.saves.list {
                ui.set_number(l, t::USER0, 1.0);
            }
            self.set(flag::BACK_TO_SAVES, false);
        }
        if let Some(i) = self.changed.take() {
            self.show_value(ui, i);
            let item = &self.user_items[i];
            if let Some(setting) = item.setting() {
                self.requests.push(Request::Apply {
                    setting,
                    value: item.value,
                    count: item.count(),
                });
            }
            self.set(flag::SETTINGS_CHANGED, true);
        }
        ui.refresh();
    }

    /// `007ce7a0(1)`: back to the game.
    fn resume(&mut self) {
        if !self.closed {
            self.requests.push(Request::Resume);
            self.closed = true;
        }
    }

    fn row(&self, tile: TileId) -> Option<Row> {
        self.rows.iter().find(|(t, _)| *t == tile).map(|(_, r)| *r)
    }

    /// The row a tile is or is in.
    fn row_of(&self, ui: &Ui, mut tile: TileId) -> Option<(TileId, Row)> {
        loop {
            if let Some(r) = self.row(tile) {
                return Some((tile, r));
            }
            tile = ui.tiles[tile].parent?;
        }
    }

    /// `007d4ce0`: a user option's line shows its value: a meter's `user0`
    /// the value; a toggle's `user0` the value's text; `user2` which arrows
    /// (wrapping: both; at the first: right; at the last: left).
    // Translated from 007d4ce0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn show_value(&mut self, ui: &mut Ui, index: usize) {
        let Some(&(tile, _)) = self.rows.iter().find(|(_, r)| *r == Row::User(index)) else {
            return;
        };
        let item = &self.user_items[index];
        let count = item.count();
        match &item.kind {
            Kind::Meter { .. } => ui.set_number(tile, t::USER0, item.value as f32),
            Kind::Toggle { values, .. } => {
                if let Some(v) = values.get(item.value as usize) {
                    ui.set_string(tile, t::USER0, v);
                }
            }
            Kind::Plain(_) => return,
        }
        let arrows = match &item.kind {
            Kind::Toggle { wrap: true, .. } => 1.0,
            _ if item.value == 0 => 2.0,
            _ if item.value == count - 1 => 0.0,
            _ => 1.0,
        };
        ui.set_number(tile, t::USER0 + 2, arrows);
    }

    /// `007d42b0`: a settings page: the options list cleared and faded
    /// in, the page's user options added with their templates (`user3` a
    /// meter's top, `user4` a toggle's arrow spacing), Action Mapping's
    /// line with only its right arrow, `_center_x` the page's centre
    /// (`user1` .. `user4` of the menu, the largest of the pages asked).
    // Translated from 007d42b0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn open_page(&mut self, ui: &mut Ui, mask: u32, has_controller: bool) {
        self.clear_list(ui, 3);
        Self::enable(ui, &self.options.clone(), true);
        self.fade(ui, id::OPTIONS, true, None);
        // Defaults (id 4) names itself (`sDefaults`).
        if let Some(d) = self.tile(id::DEFAULTS) {
            ui.set_string(d, t::STRING, &text(ui, "sDefaults"));
        }
        let menu = self.menu;
        for i in 0..self.user_items.len() {
            let item = self.user_items[i].clone();
            if item.pages & mask == 0 {
                continue;
            }
            // The controller's options only with a controller connected
            // (`XInputGetState(0)`).
            if matches!(
                item.setting(),
                Some(Setting::Controller) | Some(Setting::Rumble)
            ) && !has_controller
            {
                continue;
            }
            let Some(row) =
                Self::add_row(&mut self.options, ui, menu, item.template(), &item.label)
            else {
                continue;
            };
            self.rows.push((row, Row::User(i)));
            match &item.kind {
                Kind::Meter { ticks, .. } => ui.set_number(row, t::USER0 + 3, (*ticks - 1) as f32),
                Kind::Toggle { spacing, .. } if *spacing != 0.0 => {
                    ui.set_number(row, t::USER0 + 4, *spacing)
                }
                _ => {}
            }
            self.show_value(ui, i);
        }
        for i in 0..self.main_items.len() {
            let item = self.main_items[i].clone();
            if item.pages & mask == 0 {
                continue;
            }
            let Some(row) = Self::add_row(
                &mut self.options,
                ui,
                menu,
                "lb_meter_template",
                &item.label,
            ) else {
                continue;
            };
            self.rows.push((row, Row::Main(i)));
            ui.set_number(row, t::USER0 + 3, 16.0);
            for (child_id, shown) in [(id::LEFT, false), (id::BAR, false), (id::RIGHT, true)] {
                if let Some(c) = by_id_below(ui, row, child_id) {
                    ui.set_number(c, t::VISIBLE, f32::from(u8::from(shown)));
                    ui.set_number(c, t::TARGET, 0.0);
                }
            }
        }
        let mut centre = 0.0f32;
        for (bit, user) in [
            (page::GAMEPLAY, 1),
            (page::DISPLAY, 2),
            (page::AUDIO, 3),
            (page::CONTROLS, 4),
        ] {
            if mask & bit != 0 {
                centre = centre.max(ui.number(self.menu, t::USER0 + user));
            }
        }
        let c = Self::custom(ui, "_center_x");
        ui.set_number(self.menu, c, centre);
    }

    /// `007d48a0` / `007d4950` → `007d49a0`: a confirmation: the list
    /// faded in at (x, y) `width` wide, `confirm_question` the question,
    /// `confirm_warning` shown or not, the options of `mask`.
    // Translated from 007d49a0 (decompiled, FalloutNV.exe 1.4.0.525)
    #[allow(clippy::too_many_arguments)]
    fn open_confirm(
        &mut self,
        ui: &mut Ui,
        mask: u32,
        question: &str,
        x: f32,
        y: f32,
        width: f32,
        warning: bool,
    ) {
        self.clear_list(ui, 2);
        Self::enable(ui, &self.confirm.clone(), true);
        self.fade(ui, id::CONFIRM, true, None);
        if let Some(c) = self.tile(id::CONFIRM) {
            let (px, py) = match ui.tiles[c].parent {
                Some(p) => ui.screen_position(p),
                None => (0.0, 0.0),
            };
            ui.set_number(c, t::X, x - px);
            ui.set_number(c, t::Y, y - py);
            ui.set_number(c, t::WIDTH, width);
        }
        if let Some(q) = self.tile(id::QUESTION) {
            ui.set_string(q, t::STRING, question);
        }
        if let Some(w) = self.tile(id::WARNING) {
            ui.set_number(w, t::VISIBLE, f32::from(u8::from(warning)));
        }
        let menu = self.menu;
        for i in 0..self.main_items.len() {
            let item = self.main_items[i].clone();
            if item.pages & mask == 0 {
                continue;
            }
            if let Some(row) =
                Self::add_row(&mut self.confirm, ui, menu, "lb_item_template", &item.label)
            {
                self.rows.push((row, Row::Main(i)));
            }
        }
    }

    /// `007d48a0`: a confirmation by the main list's chosen line: x the
    /// main list's left less the width (168 with flag 1, else 80) less the
    /// screen's crop [guess: `007177c0` read as the x crop], y the line's
    /// less 60; flag 2 shows the warning.
    // Translated from 007d48a0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn confirm_by_main(&mut self, ui: &mut Ui, mask: u32, question: &str, flags: u8) {
        let width = if flags & 1 != 0 { 168.0 } else { 80.0 };
        let (mx, _) = self.main.list.map_or((0.0, 0.0), |l| ui.screen_position(l));
        let crop = ui.screen_size.safe_x;
        let x = (mx.round() - width - crop).round();
        let y = self
            .main
            .selected
            .map_or(0.0, |s| ui.screen_position(s).1)
            .round()
            - 60.0;
        self.open_confirm(ui, mask, question, x, y, width, flags & 2 != 0);
    }

    /// `007d40a0`: a confirmation for the chosen save: delete
    /// (`sConfirmDelete`, 0x800), save over (`sConfirmSave`, 0x400) or
    /// load (`sConfirmLoad`, 0x200; from the pause menu with the warning),
    /// the save list disabled and its `user0` 0, `confirm_question` the
    /// question; placed below the chosen line (one line's height, two with
    /// the warning) at the list's x less the question's `x` plus the
    /// question's width, the scroll bar's width and the list's
    /// `_text_offset`.
    // Translated from 007d40a0 (disassembly, FalloutNV.exe 1.4.0.525)
    fn confirm_save(&mut self, ui: &mut Ui) {
        let (mask, question, flags) = if self.has(flag::DELETE_MODE) {
            (page::CONFIRM_DELETE, "sConfirmDelete", 0u8)
        } else if self.has(flag::SAVE_MODE) {
            (page::CONFIRM_SAVE, "sConfirmSave", 0)
        } else {
            let f = if self.has(flag::PAUSE) { 2 } else { 0 };
            (page::CONFIRM_LOAD, "sConfirmLoad", f)
        };
        let Some(sel) = self.saves.selected else {
            return;
        };
        Self::enable(ui, &self.saves.clone(), false);
        let question = text(ui, question);
        // `007d8d60`: the chosen line marked (`_selected`), the others
        // hidden (the list's `user0` 0).
        let selected = Self::custom(ui, "_selected");
        ui.set_number(sel, selected, 1.0);
        if let Some(l) = self.saves.list {
            ui.set_number(l, t::USER0, 0.0);
        }
        if let Some(q) = self.tile(id::QUESTION) {
            ui.set_string(q, t::STRING, &question);
        }
        let lines = if flags & 2 != 0 { 2.0 } else { 1.0 };
        let (_, sy) = ui.screen_position(sel);
        let y = (sy + ui.number(sel, t::HEIGHT) * lines).round();
        let (lx, _) = self
            .saves
            .list
            .map_or((0.0, 0.0), |l| ui.screen_position(l));
        let (qx, qw) = match self.tile(id::QUESTION) {
            Some(q) => {
                let w = ui.layout(q).map_or(0.0, |l| l.width as f32);
                (ui.number(q, t::X), w)
            }
            None => (0.0, 0.0),
        };
        let bar = self.saves.scrollbar.map_or(0.0, |b| ui.number(b, t::WIDTH));
        let offset = self.saves.list.map_or(0.0, |l| {
            let o = Self::custom(ui, "_text_offset");
            ui.number(l, o)
        });
        let x = (lx - qx + qw + bar + offset).round();
        let width = if flags & 1 != 0 { 168.0 } else { 85.0 };
        self.open_confirm(ui, mask, &question, x, y, width, flags & 2 != 0);
    }
    /// `007d3b60` / `007d3c70` / `007d67b0`: the save list in the mode
    /// the flags say: saves newest first (the caller's order); in save mode
    /// "[NEW SAVE] n/1000 saves used." first; in load mode without saves
    /// "[NO SAVES FOUND]", disabled. Each line's `user0` the prefix and
    /// `user1` the place.
    // Translated from 007d3c70 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn open_saves(&mut self, ui: &mut Ui, saves: Vec<SaveEntry>) {
        if let Some(t) = self.tile(id::BACK) {
            ui.set_number(t, t::USER0, 0.0);
        }
        self.clear_list(ui, 4);
        self.save_entries = saves;
        let menu = self.menu;
        let enabled = Self::custom(ui, "_enabled");
        if self.has(flag::SAVE_MODE) {
            let user_saves = self
                .save_entries
                .iter()
                .filter(|s| s.prefix.starts_with('#'))
                .count();
            let label = format!(
                "{} {}/{} {}",
                text(ui, "sNewSave"),
                user_saves,
                MAX_SAVES,
                text(ui, "sSavesRemaining")
            );
            if let Some(row) =
                Self::add_row(&mut self.saves, ui, menu, "lb_newsave_template", &label)
            {
                self.rows.push((row, Row::Save(None)));
                if user_saves >= MAX_SAVES {
                    ui.set_number(row, enabled, 0.0);
                }
            }
        } else if self.save_entries.is_empty() {
            let label = text(ui, "sNoSaves");
            if let Some(row) =
                Self::add_row(&mut self.saves, ui, menu, "lb_newsave_template", &label)
            {
                self.rows.push((row, Row::Save(None)));
                ui.set_number(row, enabled, 0.0);
            }
        }
        for i in 0..self.save_entries.len() {
            let s = self.save_entries[i].clone();
            if let Some(row) = Self::add_row(&mut self.saves, ui, menu, "lb_saveload_template", "")
            {
                self.rows.push((row, Row::Save(Some(i))));
                ui.set_string(row, t::USER0, &s.prefix);
                let place = if s.corrupt {
                    text(ui, "sSaveGameCorrupt")
                } else {
                    s.place.clone()
                };
                ui.set_string(row, t::USER0 + 1, &place);
            }
        }
        Self::enable(ui, &self.saves.clone(), true);
        self.fade(ui, id::SAVELOAD, true, None);
        self.fade(ui, id::BACK, true, None);
    }

    /// `007d4df0`: the chosen save's details beside the list (the player,
    /// "Level n", the play time); the picture isn't made here.
    fn show_save(&mut self, ui: &mut Ui, index: Option<usize>) {
        let entry = index.and_then(|i| self.save_entries.get(i)).cloned();
        let set = |ui: &mut Ui, tile: Option<TileId>, s: Option<String>| {
            if let Some(tile) = tile {
                ui.set_number(tile, t::VISIBLE, f32::from(u8::from(s.is_some())));
                ui.set_string(tile, t::STRING, &s.unwrap_or_default());
            }
        };
        let level = entry
            .as_ref()
            .and_then(|e| e.level)
            .map(|l| format!("{} {l}", text(ui, "sLevelAbbrev")));
        let time = entry
            .as_ref()
            .filter(|e| !e.play_time.is_empty())
            .map(|e| format!("{} {}", text(ui, "sPlayTime"), e.play_time));
        set(
            ui,
            self.tile(id::PLAYER_NAME),
            entry.as_ref().map(|e| e.player.clone()),
        );
        set(ui, self.tile(id::PLAYER_LEVEL), level);
        set(ui, self.tile(id::PLAY_TIME), time);
        if let Some(d) = self.tile(id::DEFAULTS) {
            // Delete with a save highlighted (`_target`).
            let target = Self::custom(ui, "_target");
            ui.set_number(d, target, f32::from(u8::from(entry.is_some())));
            ui.set_string(d, t::STRING, &text(ui, "sDelete"));
        }
    }

    /// Back (`007d0e40`): from a confirmation, a settings page, the
    /// settings list or the save list to the page before; on the pause
    /// menu's own list, back to the game.
    // Translated from 007d0e40 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn back(&mut self, ui: &mut Ui) {
        if !Self::list_shown(ui, &self.main) {
            return;
        }
        if let Some(v) = self.tile(id::VERSION) {
            ui.set_number(v, t::VISIBLE, 0.0);
        }
        if Self::list_enabled(ui, &self.options) {
            self.close_page(ui);
            self.open_settings(ui);
            return;
        }
        if Self::list_enabled(ui, &self.confirm) {
            Self::enable(ui, &self.confirm.clone(), false);
            self.fade(ui, id::CONFIRM, false, None);
            if Self::list_shown(ui, &self.saves) {
                self.set(flag::BACK_TO_SAVES, true);
                return;
            }
        } else if Self::list_enabled(ui, &self.settings) {
            self.fade(ui, id::SETTINGS, false, None);
            Self::enable(ui, &self.settings.clone(), false);
        } else if Self::list_enabled(ui, &self.saves) {
            self.saves.select(ui, None);
            self.show_save(ui, None);
            Self::enable(ui, &self.saves.clone(), false);
            self.fade(ui, id::SAVELOAD, false, None);
        } else if Self::list_enabled(ui, &self.main) {
            if self.has(flag::PAUSE) {
                self.resume();
            }
            return;
        } else if self.has(flag::BACK_TO_SAVES) {
            return;
        }
        Self::enable(ui, &self.main.clone(), true);
        self.fade(ui, id::BACK, false, None);
        if self.has(flag::SETTINGS_CHANGED) {
            self.requests.push(Request::SavePreferences);
            self.set(flag::SETTINGS_CHANGED, false);
        }
    }

    /// `007d5f80`: the settings page closed.
    fn close_page(&mut self, ui: &mut Ui) {
        Self::enable(ui, &self.options.clone(), false);
        self.fade(ui, id::OPTIONS, false, None);
    }

    /// `007d0700`: the settings list in, with Back.
    fn open_settings(&mut self, ui: &mut Ui) {
        Self::enable(ui, &self.settings.clone(), true);
        self.fade(ui, id::SETTINGS, true, None);
        self.fade(ui, id::BACK, true, None);
    }

    /// What a main option does when clicked.
    fn act(&mut self, ui: &mut Ui, action: Action, has_controller: bool) {
        match action {
            Action::Continue => {
                if self.has(flag::PAUSE) {
                    self.resume();
                } else {
                    let q = text(ui, "sConfirmContinue");
                    self.confirm_by_main(ui, page::CONFIRM_CONTINUE, &q, 0);
                }
            }
            Action::New => {
                let q = text(ui, "sConfirmNew");
                self.confirm_by_main(ui, page::CONFIRM_NEW, &q, 0);
            }
            Action::Save | Action::Load => {
                if let Some(t) = self.tile(id::BACK) {
                    ui.set_number(t, t::USER0, 0.0);
                }
                self.set(flag::SAVE_MODE, action == Action::Save);
                let entries = std::mem::take(&mut self.save_entries);
                self.open_saves(ui, entries);
            }
            Action::Settings => self.open_settings(ui),
            Action::Help => self.requests.push(Request::Help),
            Action::Credits | Action::Downloads | Action::ActionMapping => {
                // Not on the pause menu's pages here.
            }
            Action::Quit => {
                let q = text(ui, "sConfirmQuit");
                if self.has(flag::PAUSE) {
                    self.confirm_by_main(ui, page::QUIT_PAUSE, &q, 3);
                } else {
                    self.confirm_by_main(ui, page::QUIT_TITLE, &q, 1);
                }
            }
            Action::Page(mask) => {
                Self::enable(ui, &self.settings.clone(), false);
                self.open_page(ui, mask, has_controller);
            }
            Action::YesNew => self.back(ui),
            Action::YesContinue => {
                self.requests.push(Request::LoadNewest);
                self.back(ui);
            }
            Action::YesLoad => {
                if let Some(s) = self.chosen_save.and_then(|i| self.save_entries.get(i)) {
                    self.requests.push(Request::Load(s.key.clone()));
                }
                if self.has(flag::PAUSE) {
                    self.resume();
                } else {
                    self.back(ui);
                }
            }
            Action::YesSave => {
                Self::enable(ui, &self.confirm.clone(), false);
                self.fade(ui, id::CONFIRM, false, None);
                if let Some(q) = self.tile(id::QUESTION) {
                    ui.set_string(q, t::STRING, &text(ui, "sMenuDisplayShortXBoxSaveMessage"));
                }
                self.set(flag::SAVE_ASKED, true);
            }
            Action::YesDelete => {
                if let Some(s) = self.chosen_save.and_then(|i| self.save_entries.get(i)) {
                    self.requests.push(Request::Delete(s.key.clone()));
                }
                self.back(ui);
            }
            Action::Back => self.back(ui),
            Action::MainMenu => self.requests.push(Request::MainMenu),
            Action::ExitGame => self.requests.push(Request::ExitGame),
        }
    }

    /// The click handler with whether a controller is connected (the
    /// settings' controller options).
    // Translated from 007ce9b0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn click_with(
        &mut self,
        ui: &mut Ui,
        clicked: i32,
        tile: Option<TileId>,
        now: f64,
        has_controller: bool,
    ) {
        let now_ms = now * 1000.0;
        if clicked == id::BACK {
            self.back(ui);
        }
        if clicked == id::DEFAULTS {
            if Self::list_enabled(ui, &self.saves) {
                self.set(flag::DELETE_MODE, true);
                self.chosen_save =
                    self.saves
                        .selected
                        .and_then(|s| self.row(s))
                        .and_then(|r| match r {
                            Row::Save(i) => i,
                            _ => None,
                        });
                if self.chosen_save.is_some() {
                    self.confirm_save(ui);
                }
                self.set(flag::DELETE_MODE, false);
            } else {
                // Defaults: every user option on the page back to its
                // default (`007cfdc0`).
                let shown: Vec<usize> = self
                    .rows
                    .iter()
                    .filter_map(|(_, r)| match r {
                        Row::User(i) => Some(*i),
                        _ => None,
                    })
                    .collect();
                for i in shown {
                    let item = &mut self.user_items[i];
                    if item.value != item.default {
                        item.value = item.default;
                        self.changed = Some(i);
                        self.update_change(ui);
                    }
                }
            }
        } else if clicked == id::SETTINGS && Self::list_enabled(ui, &self.options) {
            self.close_page(ui);
            self.open_settings(ui);
        }
        let Some(tile) = tile else {
            return;
        };
        if now_ms - self.last_click <= CLICK_GAP_MS {
            return;
        }
        let Some((row_tile, row)) = self.row_of(ui, tile) else {
            return;
        };
        match row {
            Row::Main(i) => {
                let Kind::Plain(action) = self.main_items[i].kind else {
                    return;
                };
                let in_confirm = self.confirm.items.iter().any(|x| x.tile == row_tile);
                let in_main = self.main.items.iter().any(|x| x.tile == row_tile);
                let in_settings = self.settings.items.iter().any(|x| x.tile == row_tile);
                if !in_confirm && action != Action::Back {
                    if in_main {
                        Self::enable(ui, &self.main.clone(), false);
                    } else if in_settings {
                        Self::enable(ui, &self.settings.clone(), false);
                    }
                }
                self.act(ui, action, has_controller);
                self.last_click = now_ms;
            }
            Row::Save(index) => {
                self.chosen_save = index;
                match index {
                    None if self.has(flag::SAVE_MODE) => {
                        if let Some(q) = self.tile(id::QUESTION) {
                            ui.set_string(
                                q,
                                t::STRING,
                                &text(ui, "sMenuDisplayShortXBoxSaveMessage"),
                            );
                        }
                        self.set(flag::SAVE_ASKED, true);
                    }
                    None => {}
                    Some(i) if self.save_entries[i].corrupt && !self.has(flag::SAVE_MODE) => {}
                    Some(_) => self.confirm_save(ui),
                }
                self.last_click = now_ms;
            }
            Row::User(i) => {
                let item = &mut self.user_items[i];
                let count = item.count();
                let steps = match (&item.kind, clicked) {
                    (_, id::LEFT) | (_, id::RIGHT) => true,
                    (Kind::Toggle { wrap: true, .. }, _) => true,
                    (Kind::Toggle { wrap: false, .. }, _) => item.value < count - 1,
                    _ => false,
                };
                if steps && count > 0 {
                    // (value + count + (not the left arrow) × 2 − 1) mod
                    // count; at a meter's ends its arrow isn't shown
                    // (`user2`), so it doesn't wrap there.
                    let up = i32::from(clicked != id::LEFT) * 2;
                    item.value = (item.value + count + up - 1).rem_euclid(count);
                    self.changed = Some(i);
                }
            }
        }
    }

    fn update_change(&mut self, ui: &mut Ui) {
        let now = self.now;
        self.update(ui, now);
    }

    /// The pointer on a meter's bar (`007cf6a0`, id 0x66): the value under
    /// the pointer, `(x − the bar's left) × top ÷ the bar's width`,
    /// rounded and kept in range.
    // Translated from 007cf6a0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn drag_meter(&mut self, ui: &mut Ui, bar: TileId, pointer_x: f32) {
        let Some((_, Row::User(i))) = self.row_of(ui, bar) else {
            return;
        };
        let (bx, _) = ui.screen_position(bar);
        let width = ui.number(bar, t::WIDTH);
        let item = &mut self.user_items[i];
        let top = item.count() - 1;
        if width <= 0.0 || top <= 0 {
            return;
        }
        let v = (((pointer_x - bx) * top as f32) / width).round() as i32;
        let v = v.clamp(0, top);
        if v != item.value {
            item.value = v;
            self.changed = Some(i);
        }
    }

    /// The Escape control (`007cf5e0` with 10): Back, 500 ms after the
    /// last, when Back can be clicked.
    // Translated from 007cf5e0 (decompiled, FalloutNV.exe 1.4.0.525)
    pub fn escape(&mut self, ui: &mut Ui, now: f64) {
        let now_ms = now * 1000.0;
        let back_target = self
            .tile(id::BACK)
            .is_some_and(|b| ui.number(b, t::TARGET) != 0.0);
        if back_target && now_ms - self.last_click > CLICK_GAP_MS {
            self.sounds.push("UIMenuCancel".to_string());
            self.back(ui);
            self.last_click = now_ms;
        }
    }
}

/// A tile with an `id` below `from`.
fn by_id_below(ui: &mut Ui, from: TileId, wanted: i32) -> Option<TileId> {
    ui.descendants(from)
        .into_iter()
        .find(|&c| ui.has(c, t::ID) && ui.number(c, t::ID) as i32 == wanted)
}

impl MenuCode for StartMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..23).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, now: f64) {
        self.click_with(ui, id, tile, now, false);
    }

    /// `007cf7e0`: a save chosen shows its details.
    fn mouseover(&mut self, ui: &mut Ui, _id: i32, tile: TileId) {
        if let Some((_, Row::Save(i))) = self.row_of(ui, tile) {
            self.show_save(ui, i);
        }
    }

    /// `007cefc0`: Left / Right on a chosen user option click its arrows;
    /// Page Up / Down move a meter by its jump.
    // Translated from 007cefc0 (decompiled, FalloutNV.exe 1.4.0.525)
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        if !(code == special::LEFT || code == special::RIGHT) {
            return false;
        }
        if !Self::list_enabled(ui, &self.options) {
            return false;
        }
        let Some(sel) = self.options.selected else {
            return false;
        };
        let arrow = if code == special::LEFT {
            id::LEFT
        } else {
            id::RIGHT
        };
        match by_id_below(ui, sel, arrow) {
            Some(a) if ui.shown(a) && ui.number(a, t::TARGET) != 0.0 => {
                self.click_with(ui, arrow, Some(a), now, false);
                true
            }
            _ => false,
        }
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![
            &mut self.confirm,
            &mut self.options,
            &mut self.saves,
            &mut self.settings,
            &mut self.main,
        ]
    }
}

/// A user option's value index from the setting's value, and back, as the
/// options' handlers work them (`StartMenu::Set…` (Xbox PDB)).
pub mod values {
    /// A 0..1 setting (volumes `007d2790`, HUD opacity `007d1750`):
    /// index = round((count − 1) × v).
    pub fn unit_to_index(v: f32, count: i32) -> i32 {
        ((count - 1) as f32 * v).round() as i32
    }

    pub fn index_to_unit(i: i32, count: i32) -> f32 {
        i as f32 / (count - 1).max(1) as f32
    }

    /// The mouse (`007d2eb0`): v = i ÷ (count − 1) × 0.0095 + 0.0005
    /// (`01076f70`, `01076f68`); the default index (count − 1) × 0.15789475
    /// (`01076f60`).
    pub const MOUSE_SCALE: f32 = 0.0095;
    pub const MOUSE_BASE: f32 = 0.0005;
    pub const MOUSE_DEFAULT: f32 = 0.157_894_75;

    pub fn mouse_to_index(v: f32, count: i32) -> i32 {
        ((count - 1) as f32 * ((v - MOUSE_BASE) / MOUSE_SCALE)).round() as i32
    }

    pub fn index_to_mouse(i: i32, count: i32) -> f32 {
        (i as f32 / (count - 1).max(1) as f32) * MOUSE_SCALE + MOUSE_BASE
    }

    /// Brightness (`007d19c0`): gamma = min + (max − min) × i ÷ (count −
    /// 1), `fGammaMin` 1.4 .. `fGammaMax` 0.6; the default from gamma 1.
    pub fn gamma_to_index(g: f32, min: f32, max: f32, count: i32) -> i32 {
        ((count - 1) as f32 * ((g - min) / (max - min))).round() as i32
    }

    pub fn index_to_gamma(i: i32, min: f32, max: f32, count: i32) -> f32 {
        (max - min) * (i as f32 / (count - 1).max(1) as f32) + min
    }

    /// An on/off option (`007d1440` …): index 1 "ON", 0 "OFF".
    pub fn bool_to_index(on: bool) -> i32 {
        i32::from(on)
    }

    pub fn index_to_bool(i: i32) -> bool {
        i == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu::Interface;
    use crate::menus::test_support;

    fn list(name: &str, idn: i32) -> String {
        test_support::list_box(name, idn, 400.0, "<copy>200</copy>")
            .replace("<_enabled>&true;</_enabled>", "<_enabled>&false;</_enabled><_alpha>0</_alpha><visible><copy src=\"me()\" trait=\"_alpha\"/><gt>0</gt></visible>")
    }

    fn start_menu_xml() -> String {
        format!(
            "<menu name=\"StartMenu\"><class>&StartMenu;</class><user1>360</user1><user2>360</user2><user3>320</user3><user4>360</user4>
               <rect name=\"NOGLOW_BRANCH\"><locus>&true;</locus>
                 <image name=\"main_title\"><id>3</id><filename>a.dds</filename></image>
                 <text name=\"main_version\"><id>10</id></text>
                 <nif name=\"pause_background\"><id>13</id></nif>
                 {}{}{}{}{}
                 <text name=\"confirm_question\"><id>8</id></text>
                 <text name=\"confirm_warning\"><id>20</id></text>
                 <hotrect name=\"main_back_button\"><id>5</id><target>&true;</target><_alpha>0</_alpha></hotrect>
                 <hotrect name=\"main_defaults_button\"><id>4</id><target>&true;</target></hotrect>
               </rect>
               <template name=\"lb_item_template\"><hotrect name=\"i\">{item}<text name=\"ListItemText\"><font>7</font><string><copy src=\"parent()\" trait=\"string\"/></string></text></hotrect></template>
               <template name=\"lb_newsave_template\"><hotrect name=\"n\">{item}<text name=\"ListItemText\"><font>7</font><string><copy src=\"parent()\" trait=\"string\"/></string></text></hotrect></template>
               <template name=\"lb_saveload_template\"><hotrect name=\"s\">{item}<height>48</height></hotrect></template>
               <template name=\"lb_toggle_template\"><hotrect name=\"tg\">{item}<height>48</height>
                 <image name=\"l\"><id>100</id><target>&true;</target><filename>a.dds</filename><width>13</width><height>23</height></image>
                 <image name=\"r\"><id>101</id><target>&true;</target><filename>a.dds</filename><x>300</x><width>13</width><height>23</height></image></hotrect></template>
               <template name=\"lb_meter_template\"><hotrect name=\"m\">{item}<height>48</height>
                 <image name=\"l\"><id>100</id><target>&true;</target><filename>a.dds</filename><width>13</width><height>23</height></image>
                 <hotrect name=\"bar\"><id>102</id><target>&true;</target><x>100</x><width>160</width><height>24</height></hotrect>
                 <image name=\"r\"><id>101</id><target>&true;</target><filename>a.dds</filename><x>300</x><width>13</width><height>23</height></image></hotrect></template>
             </menu>",
            list("main_container", 0),
            list("settings_container", 1),
            list("options_container", 2),
            list("saveload_container", 6),
            list("confirm_container", 7),
            item = test_support::LIST_ITEM,
        )
    }

    fn opened() -> (Ui, StartMenu, TileId) {
        let mut ui = test_support::ui();
        let mut m = StartMenu::new(0, &ui);
        let tile = test_support::load(&mut ui, &start_menu_xml(), &mut m);
        m.menu = tile;
        ui.set_number(tile, t::VISIBLE, 1.0);
        let read = |s: Setting, count: i32| match s {
            Setting::MasterVolume => (values::unit_to_index(1.0, count), count - 1),
            _ => (0, 0),
        };
        assert!(m.open(&mut ui, true, true, true, true, &read));
        (ui, m, tile)
    }

    fn labels(ui: &mut Ui, l: &ListBox) -> Vec<String> {
        l.items
            .iter()
            .map(|i| ui.string(i.tile, t::STRING).unwrap_or_default())
            .collect()
    }

    /// `007cbaf0`: the pause menu's list from the options on page 2, the
    /// settings pages in the settings list; the main list fades in over
    /// 0.25 s.
    #[test]
    fn the_pause_menu_lists_its_options_and_fades_in() {
        let (mut ui, mut m, _) = opened();
        assert_eq!(
            labels(&mut ui, &m.main),
            ["Continue", "Save", "Load", "Settings", "Help", "Quit"]
        );
        assert_eq!(
            labels(&mut ui, &m.settings),
            ["Gameplay", "Display", "Audio", "Controls"]
        );
        let main = m.tile(id::MAIN).unwrap();
        let alpha = ui.names.lookup("_alpha").unwrap();
        m.update(&mut ui, 0.125);
        assert!((ui.number(main, alpha) - 127.5).abs() < 1.0);
        m.update(&mut ui, 0.5);
        assert_eq!(ui.number(main, alpha), 255.0);
    }

    fn click_label(ui: &mut Ui, m: &mut StartMenu, list: u8, label: &str, now: f64) {
        let l = match list {
            0 => &m.main,
            1 => &m.settings,
            2 => &m.confirm,
            3 => &m.options,
            _ => &m.saves,
        };
        let tile = l
            .items
            .iter()
            .map(|i| i.tile)
            .find(|&t| ui.string(t, t::STRING).as_deref() == Some(label))
            .unwrap();
        let mut i = Interface::default();
        let menu = m.menu;
        i.click(ui, menu, m, tile, now);
    }

    /// Settings → Audio: the meters with their values; the right arrow
    /// steps Master down… up and the change is carried out next frame;
    /// Back goes to the settings list, again to the main list, again (on
    /// the pause menu's own list) back to the game.
    #[test]
    fn settings_pages_change_values_and_back_returns() {
        let (mut ui, mut m, _) = opened();
        m.update(&mut ui, 0.5);
        click_label(&mut ui, &mut m, 0, "Settings", 1.0);
        assert!(m.settings.enabled(&mut ui) && !m.main.enabled(&mut ui));
        click_label(&mut ui, &mut m, 1, "Audio", 2.0);
        assert_eq!(
            labels(&mut ui, &m.options),
            ["Master", "Music", "Footstep", "Voice", "Effects", "Radio"]
        );
        let master = m.options.items[0].tile;
        assert_eq!(ui.number(master, t::USER0), 25.0);
        // Left arrow: one step down, applied next frame.
        let left = by_id_below(&mut ui, master, id::LEFT).unwrap();
        m.click_with(&mut ui, id::LEFT, Some(left), 3.0, false);
        m.update(&mut ui, 3.0);
        assert_eq!(ui.number(master, t::USER0), 24.0);
        assert!(m.requests.contains(&Request::Apply {
            setting: Setting::MasterVolume,
            value: 24,
            count: 26
        }));
        m.back(&mut ui);
        assert!(m.settings.enabled(&mut ui) && !m.options.enabled(&mut ui));
        m.back(&mut ui);
        assert!(m.main.enabled(&mut ui));
        assert!(m.requests.contains(&Request::SavePreferences));
        m.back(&mut ui);
        assert!(m.closed && m.requests.contains(&Request::Resume));
    }

    /// Quit from the pause menu: Main Menu / Exit Game / Cancel with the
    /// question; Exit Game asks the program to end; Cancel goes back.
    #[test]
    fn quit_asks_with_the_games_choices() {
        let (mut ui, mut m, _) = opened();
        m.update(&mut ui, 0.5);
        click_label(&mut ui, &mut m, 0, "Quit", 1.0);
        assert_eq!(
            labels(&mut ui, &m.confirm),
            ["Main Menu", "Exit Game", "Cancel"]
        );
        let q = m.tile(id::QUESTION).unwrap();
        assert_eq!(
            ui.string(q, t::STRING).as_deref(),
            Some("Are you sure you want to quit?")
        );
        click_label(&mut ui, &mut m, 2, "Cancel", 2.0);
        assert!(m.main.enabled(&mut ui) && !m.confirm.enabled(&mut ui));
        click_label(&mut ui, &mut m, 0, "Quit", 3.0);
        click_label(&mut ui, &mut m, 2, "Exit Game", 4.0);
        assert!(m.requests.contains(&Request::ExitGame));
    }

    /// Save: "[NEW SAVE] 1/1000 saves used." then the saves; the new save
    /// shows "Saving...", asks for the save, and the menu closes 3 s on.
    /// Load: a save clicked asks "Load this game?" with the warning; Yes
    /// loads it and goes back to the game.
    #[test]
    fn saving_and_loading_through_the_lists() {
        let (mut ui, mut m, _) = opened();
        let entry = |key: &str, prefix: &str| SaveEntry {
            key: key.into(),
            prefix: prefix.into(),
            place: "Doc Mitchell's House".into(),
            player: "Courier".into(),
            level: Some(1),
            play_time: String::new(),
            corrupt: false,
        };
        m.save_entries = vec![entry("q", "QUICK"), entry("s1", "#0001")];
        m.update(&mut ui, 0.5);
        click_label(&mut ui, &mut m, 0, "Save", 1.0);
        assert_eq!(m.saves.items.len(), 3);
        assert_eq!(
            ui.string(m.saves.items[0].tile, t::STRING).as_deref(),
            Some("[NEW SAVE] 1/1000 saves used.")
        );
        assert_eq!(
            ui.string(m.saves.items[1].tile, t::USER0).as_deref(),
            Some("QUICK")
        );
        let new = m.saves.items[0].tile;
        let mut i = Interface::default();
        let menu = m.menu;
        i.click(&mut ui, menu, &mut m, new, 2.0);
        m.update(&mut ui, 2.0);
        m.update(&mut ui, 2.1);
        assert!(m.requests.contains(&Request::SaveNew));
        assert!(!m.closed);
        m.update(&mut ui, 5.1);
        assert!(m.closed);

        let (mut ui, mut m, _) = opened();
        m.save_entries = vec![entry("q", "QUICK")];
        m.update(&mut ui, 0.5);
        click_label(&mut ui, &mut m, 0, "Load", 1.0);
        let row = m.saves.items[0].tile;
        m.saves.select(&mut ui, Some(row));
        let mut i = Interface::default();
        let menu = m.menu;
        i.click(&mut ui, menu, &mut m, row, 2.0);
        assert_eq!(labels(&mut ui, &m.confirm), ["Yes", "No"]);
        let w = m.tile(id::WARNING).unwrap();
        assert_eq!(ui.number(w, t::VISIBLE), 1.0);
        click_label(&mut ui, &mut m, 2, "Yes", 3.0);
        assert!(m.requests.contains(&Request::Load("q".into())));
        assert!(m.closed);
    }

    #[test]
    fn the_settings_values_as_the_handlers_work_them() {
        assert_eq!(values::unit_to_index(0.5, 26), 13);
        assert!((values::index_to_unit(13, 26) - 0.52).abs() < 1e-6);
        assert_eq!(values::mouse_to_index(0.002, 16), 2);
        assert!((values::index_to_mouse(15, 16) - 0.01).abs() < 1e-6);
        assert_eq!(values::gamma_to_index(1.0, 1.4, 0.6, 16), 8);
        assert_eq!(values::bool_to_index(true), 1);
    }
}

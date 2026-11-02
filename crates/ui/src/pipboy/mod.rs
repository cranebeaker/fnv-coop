//! The Pip-Boy 3000: the game's three Pip-Boy menus (`menus\main\
//! stats_menu.xml` STATS, `inventory_menu.xml` ITEMS, `map_menu.xml` DATA)
//! read and worked out by the menu system, and filled the way
//! FalloutNV.exe's menu classes fill them (`StatsMenu` vtable `0106ffd4`,
//! made by `007da2c0`; `InventoryMenu` `010739b4`, `0077fc10`; `MapMenu`
//! `01074d44`, `00796b90`): the code finds its tiles by their `id`, writes
//! the values the files read through `io()` (`user5` the health text, ...),
//! makes list rows and tab buttons from the files' templates, and moves
//! between pages as keys come in.
//!
//! With `[Pipboy] bUsePipboyMode` 1 (this install) the menus aren't drawn
//! on the screen: they're drawn into a picture (1280 × 960 menu units,
//! `007fba00`) shown on the Pip-Boy model's `pipboyscreen` (see
//! [`screen`] for how that picture is made to look like a screen).
//!
//! The game state comes in as plain values ([`PipboyInput`]; `gather`
//! reads them from the engine's state).

pub mod data;
pub mod gather;
pub mod item_mod;
pub mod items;
pub mod repair;
pub mod screen;
pub mod stats;

use crate::menu::{Interface, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

pub use data::DataMenu;
pub use item_mod::ItemModMenu;
pub use items::ItemsMenu;
pub use repair::RepairMenu;
pub use stats::StatsMenu;

/// The three menus' files.
pub const STATS_FILE: &str = "menus\\main\\stats_menu.xml";
pub const ITEMS_FILE: &str = "menus\\main\\inventory_menu.xml";
pub const DATA_FILE: &str = "menus\\main\\map_menu.xml";

/// The menus' picture: the orthographic camera the rendered-menu object
/// draws them with (`007fba00`: frustum 0 .. 1280 across, 0 .. 960 down,
/// near 0, far 10000), in menu units. The Pip-Boy's screen mesh shows the
/// top left of it (its texture coordinates run 0 .. 0.75 across and 0 ..
/// 0.76 down), and the mouse's place on the screen maps back through it
/// (`007f8720`: u × 960 × 4/3, v × 960).
pub const PICTURE_SIZE: [f32; 2] = [1280.0, 960.0];

/// A stat line: a SPECIAL, a skill, a perk, a misc statistic.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatLine {
    pub name: String,
    /// Shown right of the name (`user1`; none: hidden, -1 in the file).
    pub value: Option<i32>,
    pub description: String,
    /// Its picture (`ICON`), as written.
    pub icon: Option<String>,
}

/// A reputation (`REPU`): its name, title now, and picture.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ReputationLine {
    pub name: String,
    pub title: String,
    pub icon: Option<String>,
}

/// Which tab of the ITEMS menu an item is under (the tab line's order:
/// Weapons, Apparel, Aid, Misc, Ammo).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ItemTab {
    Weapons = 0,
    Apparel = 1,
    Aid = 2,
    Misc = 3,
    Ammo = 4,
}

/// An item carried.
#[derive(Debug, Clone, PartialEq)]
pub struct ItemLine {
    /// Its form ID (for the caller to act on).
    pub form: u32,
    pub name: String,
    pub count: i32,
    pub tab: ItemTab,
    pub equipped: bool,
    /// Can be equipped or used (`_EquippableItem`).
    pub usable: bool,
    pub value: i32,
    pub weight: f32,
    /// `ICON`, as written.
    pub icon: Option<String>,
    /// The item card's numbers, those it has. (`dps`: `00645380`'s value,
    /// not worked out yet; `gather` leaves it out.)
    pub damage: Option<f32>,
    pub dps: Option<f32>,
    /// A weapon's projectiles a shot (the DPS card writes "%.1fx%d" when
    /// more than one).
    pub projectiles: u32,
    pub damage_resistance: Option<f32>,
    pub damage_threshold: Option<f32>,
    /// 0 to 1.
    pub condition: Option<f32>,
    pub strength: Option<i32>,
    /// "Ammo name (in clip/rest)".
    pub ammo: Option<String>,
    /// Apparel's weight class: 0 light, 1 medium (`BMDT` general flag
    /// 0x08), 2 heavy (0x80).
    pub weight_class: Option<u8>,
    pub effects: Option<String>,
    /// ITEMS' Repair button can be pressed on it (`00781860`,
    /// `world::repair::can_repair`).
    pub repairable: bool,
    /// A weapon with mods fitted (`ExtraWeaponModFlags`, the row's "+").
    pub modded: bool,
}

/// A quest in the DATA menu.
#[derive(Debug, Clone, PartialEq)]
pub struct QuestLine {
    pub form: u32,
    pub name: String,
    pub completed: bool,
    /// The active quest (`ForceActiveQuest`, or chosen here).
    pub active: bool,
    /// Its objectives shown, and whether each is done.
    pub objectives: Vec<(String, bool)>,
}

/// A note (`NOTE` added to the Pip-Boy).
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NoteLine {
    pub form: u32,
    pub name: String,
    pub text: String,
    /// Its kind (`DATA`, `005e8d40`): 0 a sound, 1 text, 2 a picture, 3 a
    /// voice (a holotape).
    pub kind: u8,
    /// A picture note's texture (`XNAM`).
    pub image: Option<String>,
}

/// A note's sound playing (`MapMenu` `+0x98` .. `+0xc4`, `0079a660`):
/// how long since it began and how long all of it is, in milliseconds,
/// and the note.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoteAudio {
    pub note: u32,
    pub elapsed_ms: f32,
    pub total_ms: f32,
    /// All of it has played.
    pub done: bool,
}

/// A map marker on the world map.
#[derive(Debug, Clone, PartialEq)]
pub struct MarkerLine {
    pub form: u32,
    pub name: String,
    /// Where on the map picture, 0 to 1 across and down (`0079c380`).
    pub at: [f32; 2],
    /// `TNAM` (1 city .. 14 vault).
    pub kind: u8,
    /// Can be travelled to (found).
    pub travel: bool,
}

/// The world map: the worldspace's picture (`ICON`), its usable size
/// (`MNAM`), markers, and the player's place and heading.
#[derive(Debug, Clone, PartialEq)]
pub struct WorldMapLine {
    pub picture: String,
    pub size: [f32; 2],
    pub markers: Vec<MarkerLine>,
    pub player: Option<([f32; 2], f32)>,
    /// The north-west and south-east corners (game units) the picture
    /// spans (`MNAM`'s cells), for turning a point back into a place.
    pub corners: [[f32; 2]; 2],
    /// The player's own marker on the picture, when in this worldspace.
    pub custom: Option<[f32; 2]>,
    /// The active quest's targets on the picture (`0079e0a0`).
    pub quest: Vec<[f32; 2]>,
}

/// DATA › Local Map (`world::local_map`): the grid of tile pictures made so
/// far (each with its 17 × 17 corners' fog of war, rows northward), and the
/// markers, as places 0..1 on the map.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LocalMapLine {
    /// Changes when the map is made again (a new place): the map is
    /// zoomed and centred anew.
    pub key: u64,
    /// Tiles a side and a tile's size on the map (1024 outdoors, 2048
    /// indoors, `0079ffb0`).
    pub grids: i32,
    pub tile_px: f32,
    pub tiles: Vec<LocalTile>,
    /// All the tiles drawn: the map is shown (`0079d410`).
    pub done: bool,
    /// The player: place and the arrow's `rotateangle`.
    pub player: Option<([f32; 2], f32)>,
    pub doors: Vec<LocalDoor>,
    pub quests: Vec<[f32; 2]>,
    pub custom: Option<[f32; 2]>,
}

/// One tile's picture on the local map.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LocalTile {
    pub gx: i32,
    pub gy: i32,
    /// The picture's name for the drawing (made by the caller).
    pub picture: String,
    /// The 17 × 17 corners' alpha (`seen / 4`), row by row northward.
    pub fog: Vec<f32>,
}

/// A door's marker on the local map.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct LocalDoor {
    pub at: [f32; 2],
    pub name: String,
    /// 0..255 (the fog of war at the door).
    pub alpha: f32,
}
/// A radio station's row (`0079bea0`): the station's reference, its name,
/// whether it's in range now (else its text at alpha 128) and whether it's
/// the one the radio plays (`_selected`: the filled square).
#[derive(Debug, Clone, PartialEq)]
pub struct StationLine {
    pub reference: u32,
    pub name: String,
    pub in_range: bool,
    pub tuned: bool,
}

/// What the Pip-Boy shows, from the game's state.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PipboyInput {
    pub name: String,
    pub level: i32,
    /// The XP text: "XP/next level" (`%d/%d`), or none at the top level
    /// (the code writes `sStatsXPMax`).
    pub xp: Option<(i32, i32)>,
    pub health: (f32, f32),
    pub action_points: (f32, f32),
    /// Head, torso, left arm, right arm, left leg, right leg (actor values
    /// 25 to 30), 0 to 100.
    pub limbs: [f32; 6],
    pub rads: f32,
    pub rad_resistance: f32,
    pub hardcore: bool,
    pub effects: Vec<(String, String)>,
    pub stimpaks: i32,
    pub doctors_bags: i32,
    pub radaway: i32,
    pub radx: i32,
    /// The aid items' forms (Stimpak, Doctor's Bag, RadAway, Rad-X: default
    /// objects 0, 21, 3, 2), for the Status page's buttons, and their names
    /// (the buttons' `user0`, `007da2c0`).
    pub aid: [Option<u32>; 4],
    pub aid_names: [String; 4],
    pub special: Vec<StatLine>,
    pub skills: Vec<StatLine>,
    pub perks: Vec<StatLine>,
    pub general: Vec<StatLine>,
    /// Karma: 0 good, 1 neutral, 2 bad, 3 very good, 4 very evil (the
    /// order of `007dd090`'s pictures), the alignment's name and the
    /// karmic title.
    pub karma_band: u8,
    pub alignment: String,
    pub karma_title: String,
    pub reputations: Vec<ReputationLine>,
    pub items: Vec<ItemLine>,
    /// The keys carried (`KEYM`), for the keyring.
    pub keys: Vec<ItemLine>,
    pub caps: i32,
    pub weight: (f32, f32),
    pub damage_resistance: f32,
    pub damage_threshold: f32,
    pub location: String,
    pub date_time: String,
    pub quests: Vec<QuestLine>,
    pub notes: Vec<NoteLine>,
    pub world_map: Option<WorldMapLine>,
    /// DATA › Radio's rows (`world::radio::Radio::rows`).
    pub stations: Vec<StationLine>,
    /// DATA › Local Map, when the caller has made it.
    pub local_map: Option<LocalMapLine>,
    /// The items on the hot keys 1 to 8 (carried ones).
    pub hotkeys: [Option<u32>; 8],
    /// A note's sound playing (the caller's).
    pub note_audio: Option<NoteAudio>,
}

/// The three Pip-Boy menus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Section {
    #[default]
    Stats,
    Items,
    Data,
}

/// A key as the menus see it: the codes the interface manager hands the
/// menus' key handlers (`0070f6e0` → `00717f80` → the menu's slot 0x38,
/// `007db680` and kin). On a PC keyboard (`007154b0`, `0070c4a0`): the
/// arrows give 1 up, 2 down, 4 left, 3 right, and with Shift held left and
/// right give 0x0D / 0x0E (the previous / next menu, the pad's triggers);
/// Enter gives the A button (-2: the chosen tile), with Shift the X
/// button (0x0B), with Alt the Y button (0x0C); Page Up / Page Down the
/// bumpers (0x0F / 0x10); other keys their letter, looked up as the menu's
/// `_PCButton_<letter>` trait.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    /// The next page or tab (code 3).
    Right,
    /// The previous one (code 4).
    Left,
    /// The next or previous menu (STATS, ITEMS, DATA: codes 0x0E / 0x0D).
    NextSection,
    PrevSection,
    /// The A button (Enter): equip, use, travel, make active.
    Activate,
    /// The X and Y buttons (Shift + Enter, Alt + Enter).
    ButtonX,
    ButtonY,
    /// Page Up and Page Down: the pad's bumpers (codes 0x0F / 0x10,
    /// `007154b0` DIK 0xC9 / 0xD1): zoom DATA's maps.
    PageUp,
    PageDown,
    /// A letter: the button the menu's `_PCButton_<letter>` names
    /// (`stats_stimpak_button` for S), clicked when it shows and can be
    /// clicked (`0070c4a0`).
    Letter(char),
}

/// What the Pip-Boy asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Play a sound record (by editor ID).
    Sound(String),
    /// A radio station's row clicked (`00796fd0` case 0x19): in range,
    /// the radio is turned off, then on and tuned to it unless it was the
    /// one playing (`world::radio::Radio::click_row`).
    Radio(u32),
    /// Equip or take off an item, or use it (aid, books).
    Equip(u32),
    Use(u32),
    /// Drop an item (`00780140` case 7): the game checks it can, then asks
    /// how many above `iInventoryAskQuantityAt`.
    Drop(u32),
    /// Fast travel to a map marker (its reference) asked for: the game
    /// checks the player can travel, then asks "Do you want to travel to
    /// ...?" (`00796fd0` case 0x1a).
    Travel(u32),
    /// The player's own marker asked for at this point of the world map's
    /// picture (0 to 1; `00796fd0` case 0x0c): the game asks to set it, or
    /// to move, remove or leave the one there.
    PlaceMarker([f32; 2]),
    /// Make a quest the active one.
    ActiveQuest(u32),
    /// Put an item on a hot key (0 to 7; `007019e0`, the item off any other).
    SetHotkey {
        slot: usize,
        item: u32,
    },
    /// A notice on the HUD (`007052f0`; a refusal's text).
    Notice(String),
    /// Play a sound or voice note's audio (`00796fd0` case 0x18), or stop
    /// the one playing (`00798ad0`).
    PlayNote(u32),
    StopNote,
    /// The scroll knob turns a notch (`007f8610(0, ±fScrollKnobIncrement,
    /// fScrollKnobRate, ...)`, with the `UIPipBoyScroll` click): `down`
    /// when the choice moved to a later row (or the map zoomed in).
    ScrollKnob {
        down: bool,
    },
    /// Open the repair screen on an item (ITEMS' Repair, `00780140` case
    /// 8): the game answers with [`Pipboy::open_repair`].
    OpenRepair(u32),
    /// Mend `chosen` with one `part` (`007b5d80`): the game answers with
    /// [`Pipboy::repaired`].
    Repair {
        chosen: u32,
        part: u32,
    },
    /// Open the weapon mod screen on a weapon (ITEMS' Mod, `00780140` case
    /// 0x13): the game answers with [`Pipboy::open_item_mod`].
    OpenItemMod(u32),
    /// Fit a mod from the player's things to the weapon (`00783af0`): the
    /// game answers with [`Pipboy::item_modded`].
    FitMod {
        weapon: u32,
        item: u32,
    },
    /// Ask the tutorial manager for a help message (`ShowMessage(id,
    /// menu, delay)`, `00718630`): ITEMS' Apparel and Ammo tabs
    /// (`00780140`).
    Tutorial {
        id: u8,
        menu: i32,
        delay: u32,
    },
}

/// A tile found by its `id` in a menu (the menu objects keep their tiles
/// by id: `StatsMenu` 60 of them from `+0x90`, checked by `007db340`).
pub fn by_id(ui: &Ui, root: TileId, id: i32) -> Option<TileId> {
    let mut stack = vec![root];
    while let Some(tile) = stack.pop() {
        if ui.tiles[tile]
            .traits
            .get(&t::ID)
            .is_some_and(|tr| tr.value.number == id as f32 && tr.actions.is_empty())
        {
            return Some(tile);
        }
        for &c in ui.tiles[tile].children.iter().rev() {
            stack.push(c);
        }
    }
    None
}

/// A custom trait's number.
pub(crate) fn trait_id(ui: &mut Ui, name: &str) -> i32 {
    ui.names.lookup_or_add(name).unwrap_or(0)
}

/// A setting's text.
pub(crate) fn text(ui: &Ui, name: &str) -> String {
    ui.setting_text(name).unwrap_or_default()
}

/// Reads a menu file (with its prefabs) and puts it on the screen hidden.
pub(crate) fn load_menu(
    ui: &mut Ui,
    file: &str,
    read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
) -> Result<TileId, String> {
    let bytes = read(file).ok_or_else(|| format!("{file} not found"))?;
    ui.load_menu(&bytes, read)
        .map_err(|e| format!("{file}: {e}"))
}

/// The Pip-Boy: its three menus and which shows, and the repair screen
/// opened from ITEMS.
pub struct Pipboy {
    pub stats: StatsMenu,
    pub items: ItemsMenu,
    pub data: DataMenu,
    /// Traits the code animates (`ui::anim`), and the limb blinking now.
    pub anims: crate::anim::Animations,
    blinking: Option<usize>,
    pub section: Section,
    /// The repair screen (none when its file can't be read).
    pub repair: Option<RepairMenu>,
    /// It's up, over ITEMS.
    pub repairing: bool,
    /// The weapon mod screen (none when its file can't be read).
    pub item_mod: Option<ItemModMenu>,
    /// It's up, over ITEMS.
    pub modding: bool,
    /// The interface's pointer state over the shown menu (the tile under
    /// the pointer, the one the button went down on, a drag).
    pub interface: Interface,
    /// Where the pointer last was on the screen.
    last_at: Option<[f32; 2]>,
}

/// The menu classes' numbers (`Menu` vtable slot `+0x34`): StatsMenu
/// 1003 (0x3eb), InventoryMenu 1002 (0x3ea), MapMenu 1023 (0x3ff), as
/// `00717920` and `00704c10` / `007048f0` / `00704170` name them.
pub const STATS_CLASS: i32 = 0x3eb;
pub const ITEMS_CLASS: i32 = 0x3ea;
pub const DATA_CLASS: i32 = 0x3ff;
/// `RepairMenu` 1035 (`007b5720`) and `ItemModMenu` 1061 (`00783640`),
/// over ITEMS.
pub const REPAIR_CLASS: i32 = 0x40b;
pub const ITEM_MOD_CLASS: i32 = 0x425;

/// The left mouse button this frame, as `0070c4a0` reads it
/// (`00a23a50(0, 0 / 1 / 2)`: held, gone down, come up).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Button {
    pub down: bool,
    pub pressed: bool,
    pub released: bool,
}

/// The shown menu's code as the interface sees it (the `Menu` vtable's
/// click, mouse-over and mouse-off slots), with what it asks of the game.
/// The repair or mod screen, when one is up over ITEMS, is the menu on top
/// and gets the pointer instead.
struct Code<'a> {
    section: Section,
    stats: &'a mut StatsMenu,
    items: &'a mut ItemsMenu,
    data: &'a mut DataMenu,
    repair: Option<&'a mut RepairMenu>,
    item_mod: Option<&'a mut ItemModMenu>,
    input: &'a PipboyInput,
    actions: Vec<Action>,
    /// The repair or mod screen's Cancel was clicked (`007b5b40` /
    /// `007838a0` case 0xc): the Pip-Boy closes it.
    cancel: bool,
}

// The classes of the screens over ITEMS (`GetClass`, vtable `+0x34`:
// `007b54b0` 1035, `007833d0` 1061): `REPAIR_CLASS`, `ITEM_MOD_CLASS` above.

impl MenuCode for Code<'_> {
    fn class(&self) -> i32 {
        if self.repair.is_some() {
            return REPAIR_CLASS;
        }
        if self.item_mod.is_some() {
            return ITEM_MOD_CLASS;
        }
        match self.section {
            Section::Stats => STATS_CLASS,
            Section::Items => ITEMS_CLASS,
            Section::Data => DATA_CLASS,
        }
    }

    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        if let Some(r) = self.repair.as_deref_mut() {
            if id == repair::CANCEL {
                self.cancel = true;
            } else {
                self.actions.extend(r.click(ui, id, tile));
            }
            return;
        }
        if let Some(m) = self.item_mod.as_deref_mut() {
            if id == item_mod::CANCEL {
                self.cancel = true;
            } else {
                self.actions.extend(m.click(ui, id, tile));
            }
            return;
        }
        let input = self.input;
        let out = match self.section {
            Section::Stats => self.stats.click(ui, id, input),
            Section::Items => self.items.click(ui, id, tile, input),
            Section::Data => self.data.click(ui, id, tile, input),
        };
        self.actions.extend(out);
    }

    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if let Some(r) = self.repair.as_deref_mut() {
            self.actions.extend(r.mouseover(ui, id, tile));
            return;
        }
        if let Some(m) = self.item_mod.as_deref_mut() {
            self.actions.extend(m.mouseover(ui, id, tile));
            return;
        }
        let input = self.input;
        let out = match self.section {
            Section::Stats => self.stats.mouseover(ui, tile, input),
            Section::Items => self.items.mouseover(ui, id, tile, input),
            Section::Data => self.data.mouseover(ui, id, tile, input),
        };
        self.actions.extend(out);
    }

    fn unmouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if let Some(r) = self.repair.as_deref_mut() {
            r.unmouseover(ui, id, tile);
            return;
        }
        if let Some(m) = self.item_mod.as_deref_mut() {
            m.unmouseover(ui, id, tile);
            return;
        }
        match self.section {
            Section::Items => self.items.unmouseover(ui, id, tile),
            Section::Stats => self.stats.unmouseover(ui, tile),
            Section::Data => {}
        }
    }

    /// The wheel (`+0x28`): DATA zooms its maps (`0079c530`); STATS',
    /// ITEMS', the repair and the mod screen's slots are the base menu's
    /// (`8d0600`, nothing).
    fn wheel(&mut self, ui: &mut Ui, _id: i32, _tile: TileId, delta: i32) {
        if self.repair.is_none() && self.item_mod.is_none() && self.section == Section::Data {
            let out = self.data.wheel(ui, delta);
            self.actions.extend(out);
        }
    }
}

impl Pipboy {
    /// Reads the three menus and sets them up as their makers do.
    pub fn load(
        ui: &mut Ui,
        read: &mut dyn FnMut(&str) -> Option<Vec<u8>>,
    ) -> Result<Pipboy, String> {
        let stats = StatsMenu::load(ui, read)?;
        let items = ItemsMenu::load(ui, read)?;
        let data = DataMenu::load(ui, read)?;
        let repair = RepairMenu::load(ui, read).ok();
        let item_mod = ItemModMenu::load(ui, read).ok();
        let mut p = Pipboy {
            stats,
            items,
            data,
            anims: crate::anim::Animations::default(),
            blinking: None,
            section: Section::Stats,
            interface: Interface::default(),
            last_at: None,
            repair,
            repairing: false,
            item_mod,
            modding: false,
        };
        p.show(ui, Section::Stats);
        Ok(p)
    }

    /// The shown menu's tile.
    pub fn menu(&self) -> TileId {
        if let Some(r) = self.repair.as_ref().filter(|_| self.repairing) {
            return r.menu;
        }
        if let Some(m) = self.item_mod.as_ref().filter(|_| self.modding) {
            return m.menu;
        }
        match self.section {
            Section::Stats => self.stats.menu,
            Section::Items => self.items.menu,
            Section::Data => self.data.menu,
        }
    }

    /// The shown menu's class number (STATS, ITEMS, DATA).
    pub fn class(&self) -> i32 {
        match self.section {
            Section::Stats => STATS_CLASS,
            Section::Items => ITEMS_CLASS,
            Section::Data => DATA_CLASS,
        }
    }

    /// The class of the menu on top: the repair or mod screen over ITEMS,
    /// else the shown menu's.
    pub fn top_class(&self) -> i32 {
        if self.repair.is_some() && self.repairing {
            REPAIR_CLASS
        } else if self.item_mod.is_some() && self.modding {
            ITEM_MOD_CLASS
        } else {
            self.class()
        }
    }

    /// Shows one of the three (the others hidden). The pointer's tiles
    /// were the old menu's: the interface lets them go (the menu that
    /// closes takes its tiles with it) and picks afresh.
    pub fn show(&mut self, ui: &mut Ui, section: Section) {
        // The repair and mod screens close.
        self.close_repair(ui);
        self.close_item_mod(ui);
        if section != self.section {
            for tile in [self.interface.over, self.interface.focus]
                .into_iter()
                .flatten()
            {
                ui.set_number(tile, t::MOUSEOVER, 0.0);
            }
            self.interface = Interface::default();
            self.data.cursor_hidden = false;
        }
        self.section = section;
        if section == Section::Data {
            self.data.shown();
        }
        for (menu, on) in [
            (self.stats.menu, section == Section::Stats),
            (self.items.menu, section == Section::Items),
            (self.data.menu, section == Section::Data),
        ] {
            ui.set_number(menu, t::VISIBLE, if on { 1.0 } else { 0.0 });
        }
    }

    /// The shown list's last row under the pointer (each menu's
    /// `listindex` it compares, `011da7d8` and kin) and its chosen row, for
    /// the scroll knob's direction.
    fn list_index(&self) -> (Option<usize>, Option<usize>) {
        let hovered = match self.section {
            Section::Items => self.items.hovered,
            Section::Data => self.data.hovered,
            Section::Stats => self.stats.hovered[self.stats.page.min(4)],
        };
        (hovered, self.chosen_index())
    }

    fn chosen_index(&self) -> Option<usize> {
        match self.section {
            Section::Items => self.items.list.selected,
            Section::Data => match self.data.tab {
                2 => self.data.quests.selected,
                3 => self.data.notes.selected,
                4 => self.data.radio.selected,
                _ => None,
            },
            Section::Stats => match self.stats.page {
                1 => self.stats.special.selected,
                2 => self.stats.skills.selected,
                3 => self.stats.perks.selected,
                4 => self.stats.general.selected,
                _ => self.stats.effects.selected,
            },
        }
    }

    /// With the knob's click (`UIPipBoyScroll`) the scroll knob turns, a
    /// notch one way for a later row than `before`, the other for an
    /// earlier one (`007b6120` and the other menus' mouse-overs:
    /// +`fScrollKnobIncrement` when the new `listindex` is the greater).
    fn route(
        &mut self,
        mut out: Vec<Action>,
        before: (Option<usize>, Option<usize>),
    ) -> Vec<Action> {
        let clicked = out
            .iter()
            .any(|a| matches!(a, Action::Sound(s) if s == "UIPipBoyScroll"));
        let turned = out.iter().any(|a| matches!(a, Action::ScrollKnob { .. }));
        if clicked && !turned {
            let after = self.list_index();
            // The pointer's row when it moved, else the chosen row (keys).
            let (b, a) = if after.0 != before.0 {
                (before.0, after.0)
            } else {
                (before.1, after.1)
            };
            out.push(Action::ScrollKnob {
                down: match b {
                    None => true,
                    Some(b) => a.is_some_and(|a| a > b),
                },
            });
        }
        out
    }

    /// The number keys held (hot keys 1 to 8), every frame while ITEMS
    /// shows (`00781ba0` runs with the inventory menu on top): the hot key
    /// wheel.
    pub fn hotkey_keys(&mut self, ui: &mut Ui, down: [bool; 8], input: &PipboyInput) {
        if self.section == Section::Items {
            self.items.hotkey_keys(ui, down, input);
            ui.refresh();
        }
    }

    /// Every frame, `now` in seconds: the limb healing mode aims at blinks
    /// (`007dbfa0` → `007dbfe0` → `007dc040`: its tiles' alpha from 255 to
    /// 0 and back each second, `00a07c60` mode 1; the one it leaves stops
    /// at 255), and the animated traits move (`00a080d0`).
    pub fn frame(&mut self, ui: &mut Ui, now: f64) {
        let aimed = self
            .stats
            .part
            .filter(|_| self.stats.healing && self.section == Section::Stats);
        if aimed != self.blinking {
            if let Some(old) = self.blinking {
                for tile in stats::limb_tiles(ui, self.stats.menu, old) {
                    self.anims.stop(tile, t::ALPHA);
                    ui.set_number(tile, t::ALPHA, 255.0);
                }
            }
            if let Some(new) = aimed {
                for tile in stats::limb_tiles(ui, self.stats.menu, new) {
                    self.anims.pulse(tile, t::ALPHA, 255.0, 0.0, 1.0, now);
                }
            }
            self.blinking = aimed;
        }
        self.anims.step(ui, now);
        ui.refresh();
    }

    /// Fills all three from the game's state.
    pub fn fill(&mut self, ui: &mut Ui, input: &PipboyInput) {
        self.stats.fill(ui, input);
        self.items.fill(ui, input);
        self.data.fill(ui, input);
        self.data.note_audio(ui, input.note_audio);
        ui.refresh();
    }

    /// A key: moves within the shown menu or to the next one. Returns what
    /// the game should do (sounds, equipping, travel).
    pub fn key(&mut self, ui: &mut Ui, key: Key, input: &PipboyInput) -> Vec<Action> {
        let before = self.list_index();
        let out = self.key_inner(ui, key, input);
        let out = self.route(out, before);
        ui.refresh();
        out
    }

    fn key_inner(&mut self, ui: &mut Ui, key: Key, input: &PipboyInput) -> Vec<Action> {
        let mut out = Vec::new();
        // The repair screen has the keys while it's up: its list, Enter,
        // and its `_PCButton_E` (Cancel); the rest do nothing.
        // The mod screen likewise: its list, Enter, and E (Cancel).
        if self.modding {
            match key {
                Key::Letter(c) => {
                    if self.pc_button(ui, c) == Some(item_mod::CANCEL) {
                        out.extend(self.cancel_item_mod(ui));
                    }
                }
                _ => {
                    if let Some(m) = self.item_mod.as_mut() {
                        out.extend(m.key(ui, key));
                    }
                }
            }
            ui.refresh();
            return out;
        }
        if self.repairing {
            match key {
                Key::Letter(c) => {
                    if self.pc_button(ui, c) == Some(repair::CANCEL) {
                        out.extend(self.cancel_repair(ui));
                    }
                }
                _ => {
                    if let Some(r) = self.repair.as_mut() {
                        out.extend(r.key(ui, key));
                    }
                }
            }
            ui.refresh();
            return out;
        }
        match key {
            // Round the three (`007db680`, `00782190`, `00799790`: STATS
            // goes back to DATA and on to ITEMS, DATA on to STATS).
            Key::NextSection | Key::PrevSection => {
                let order = [Section::Stats, Section::Items, Section::Data];
                let at = order.iter().position(|&s| s == self.section).unwrap_or(0);
                let next = if key == Key::NextSection {
                    (at + 1) % 3
                } else {
                    (at + 2) % 3
                };
                self.show(ui, order[next]);
                // The tab knob turning (`007fa0f0`).
                out.push(Action::Sound("UIPipBoyTab".into()));
            }
            Key::Letter(c) => {
                if let Some(id) = self.pc_button(ui, c) {
                    out.extend(self.click(ui, id, input));
                }
            }
            Key::ButtonX | Key::ButtonY if self.section == Section::Items => {
                match self.items.pad_button(ui, key) {
                    items::PadPress::Click(id) => out.extend(self.click(ui, id, input)),
                    items::PadPress::Refused => out.push(Action::Sound("UIMenuCancel".into())),
                    items::PadPress::Nothing => {}
                }
            }
            _ => match self.section {
                Section::Stats => out.extend(self.stats.key(ui, key, input)),
                Section::Items => out.extend(self.items.key(ui, key, input)),
                Section::Data => out.extend(self.data.key(ui, key, input)),
            },
        }
        out
    }

    /// The `id` of the button a letter presses (`0070c4a0`): the shown
    /// menu's `_PCButton_<letter>` trait names a tile; it's pressed when it
    /// shows (`visible`) and can be (`target`).
    fn pc_button(&self, ui: &mut Ui, c: char) -> Option<i32> {
        let menu = self.menu();
        let name = format!("_PCButton_{}", c.to_ascii_uppercase());
        let trait_id = ui.names.lookup(&name)?;
        let tile_name = ui.string(menu, trait_id)?;
        let tile = ui.find(menu, tile_name.trim())?;
        (ui.number(tile, t::VISIBLE) != 0.0 && ui.number(tile, t::TARGET) != 0.0)
            .then(|| ui.number(tile, t::ID) as i32)
    }

    /// A button of the shown menu pressed (the menus' click handlers, slot
    /// 0x0C: `007db380` STATS, `00780140` ITEMS, `00796fd0` DATA). DATA's
    /// lettered button (R: `MM_ButtonY`) does what isn't here.
    pub fn click(&mut self, ui: &mut Ui, id: i32, input: &PipboyInput) -> Vec<Action> {
        let before = self.list_index();
        let ((), out) = self.with_code(ui, input, |_, code, _, ui| {
            code.click(ui, id, None, 0.0);
        });
        self.route(out, before)
    }

    /// Runs the interface over the shown menu with its code.
    fn with_code<R>(
        &mut self,
        ui: &mut Ui,
        input: &PipboyInput,
        run: impl FnOnce(&mut Interface, &mut Code, TileId, &mut Ui) -> R,
    ) -> (R, Vec<Action>) {
        let menu = self.menu();
        let mut interface = std::mem::take(&mut self.interface);
        let (repairing, modding) = (self.repairing, self.modding);
        let mut code = Code {
            section: self.section,
            stats: &mut self.stats,
            items: &mut self.items,
            data: &mut self.data,
            repair: self.repair.as_mut().filter(|_| repairing),
            item_mod: self.item_mod.as_mut().filter(|_| modding && !repairing),
            input,
            actions: Vec::new(),
            cancel: false,
        };
        let r = run(&mut interface, &mut code, menu, ui);
        let mut actions = std::mem::take(&mut code.actions);
        let cancel = code.cancel;
        self.interface = interface;
        if cancel {
            if repairing {
                actions.extend(self.cancel_repair(ui));
            } else if modding {
                actions.extend(self.cancel_item_mod(ui));
            }
        }
        (r, actions)
    }

    /// The mouse over the Pip-Boy (`0070c4a0` with the Pip-Boy up): `at` is
    /// the pointer on the menus' picture in menu units (`007f8720`: the
    /// screen's texture coordinates × 1280 × 960), or none when it's off
    /// the screen, when no tile is under it (`007126c0` gives none with a
    /// Pip-Boy menu on top and the cursor off the screen). Then the
    /// interface's own rules (`ui::menu::Interface::pointer`): the tile
    /// under it moused over, a press and release on the same tile a click,
    /// a `draggable` tile dragged. On DATA's map tabs the highlight box
    /// follows the pointer (`0079a130`). Returns what the game should do
    /// (sounds, the interface's `clicksound` / `mouseoversound` included).
    pub fn pointer(
        &mut self,
        ui: &mut Ui,
        at: Option<[f32; 2]>,
        button: Button,
        now: f64,
        input: &PipboyInput,
    ) -> Vec<Action> {
        self.pointer_with_right(ui, at, button, Button::default(), false, now, input)
    }

    /// [`Pipboy::pointer`] with the right button too, and whether the
    /// Pip-Boy control (Tab) is held. Then the shown menu's every-frame
    /// code reads the right button going down (`00a23a50(1, 1)`) unless
    /// the Pip-Boy control is down (`00a24660(0xe, 1)`): on ITEMS it drops
    /// the chosen item (`00781ba0`: click 7, when an item is chosen), on
    /// DATA's world map with the pointer over the map it asks for the
    /// player's marker there (`0079a130`: click 0x0c).
    #[allow(clippy::too_many_arguments)]
    pub fn pointer_with_right(
        &mut self,
        ui: &mut Ui,
        at: Option<[f32; 2]>,
        button: Button,
        right: Button,
        pipboy_control_down: bool,
        now: f64,
        input: &PipboyInput,
    ) -> Vec<Action> {
        // The repair or mod screen up over ITEMS is the menu on top: it
        // gets the pointer (`Code`), and ITEMS' every-frame code (the right
        // button's drop) doesn't run.
        let over_items = self.repairing || self.modding;
        if button.pressed && self.section == Section::Data {
            self.data.pressed(ui);
        }
        let before = self.list_index();
        // Off the screen nothing is picked: a point no tile covers. A tile
        // being dragged keeps the last point on the screen meanwhile (the
        // map stops where the pointer left it, not flung off by a point
        // far away); nothing is known of the game's own, which follows its
        // cursor.
        if at.is_some() {
            self.last_at = at;
        }
        let held = at.or(self.last_at.filter(|_| self.interface.dragging.is_some()));
        let [x, y] = held.unwrap_or([-1.0e6, -1.0e6]);
        let ((), mut out) = self.with_code(ui, input, |interface, code, menu, ui| {
            interface.pointer(
                ui,
                menu,
                code,
                x,
                y,
                button.down,
                button.pressed,
                button.released,
                now,
            );
        });
        if self.section == Section::Data {
            out.extend(self.data.pointer_moved(ui, at, input));
        }
        if right.pressed && !pipboy_control_down && !over_items {
            match self.section {
                Section::Items => out.extend(self.items.click(ui, items::DROP_ID, None, input)),
                Section::Data => out.extend(self.data.right_pressed(ui, at)),
                _ => {}
            }
        }
        out.extend(self.interface_sounds());
        let out = self.route(out, before);
        ui.refresh();
        out
    }

    /// The mouse wheel turned `notches` (away from the player positive)
    /// over the Pip-Boy (`0070c4a0`: the nearest `wheelable` tile from the
    /// one under the pointer gets `wheelmoved`, the list boxes' scroll bars
    /// read it).
    pub fn wheel(&mut self, ui: &mut Ui, notches: i32, input: &PipboyInput) -> Vec<Action> {
        let before = self.list_index();
        let ((), mut out) = self.with_code(ui, input, |interface, code, menu, ui| {
            interface.wheel(ui, menu, code, notches);
        });
        out.extend(self.interface_sounds());
        let out = self.route(out, before);
        ui.refresh();
        out
    }

    /// The pad's sticks (left stick up / down, right stick x / y, as the
    /// 360 pad's -32767 .. 32767) over the Pip-Boy: DATA's maps zoom and
    /// pan with them (`00799790`).
    pub fn sticks(&mut self, ui: &mut Ui, left_y: i32, right: [i32; 2]) -> Vec<Action> {
        if self.repairing || self.modding || self.section != Section::Data {
            return Vec::new();
        }
        let out = self.data.sticks(ui, left_y, right);
        ui.refresh();
        out
    }

    /// The interface's own sounds (`clicksound`, `mouseoversound`).
    fn interface_sounds(&mut self) -> Vec<Action> {
        self.interface
            .effects
            .drain(..)
            .map(|e| match e {
                crate::menu::Effect::Sound(name) => Action::Sound(name),
            })
            .collect()
    }

    /// One of the Pip-Boy's own buttons pressed with the mouse (`007f8720`
    /// picks `PipBoyButton01` .. `03` on the model: 1 STATS, 2 ITEMS, 3
    /// DATA; `0070c4a0` then plays `UIMenuMode` and opens that menu,
    /// `00704c10` / `007048f0` / `00704170`).
    pub fn press_section(&mut self, ui: &mut Ui, button: usize) -> Vec<Action> {
        let section = match button {
            1 => Section::Stats,
            2 => Section::Items,
            3 => Section::Data,
            _ => return Vec::new(),
        };
        self.show(ui, section);
        ui.refresh();
        vec![Action::Sound("UIMenuMode".into())]
    }

    /// Whether the game's cursor is hidden now (DATA's map under it).
    pub fn cursor_hidden(&self) -> bool {
        self.section == Section::Data && self.data.cursor_hidden
    }

    /// The pointer's tiles were the menu's under the screen that comes or
    /// goes: they're let go and picked afresh on the next frame (as
    /// [`Pipboy::show`] does between the three menus).
    fn let_pointer_go(&mut self, ui: &mut Ui) {
        for tile in [self.interface.over, self.interface.focus]
            .into_iter()
            .flatten()
        {
            ui.set_number(tile, t::MOUSEOVER, 0.0);
        }
        self.interface = Interface::default();
    }

    /// Opens the repair screen over ITEMS with what the game worked out
    /// (the first line chosen only with a pad, `_Has360Controller`).
    /// Returns what the game should do (the scroll knob).
    pub fn open_repair(&mut self, ui: &mut Ui, input: repair::RepairInput) -> Vec<Action> {
        if self.repair.is_none() {
            return Vec::new();
        }
        self.let_pointer_go(ui);
        let pad = crate::game::has_pad(ui);
        ui.set_number(self.items.menu, t::VISIBLE, 0.0);
        let out = match self.repair.as_mut() {
            Some(r) => r.open(ui, input, pad),
            None => Vec::new(),
        };
        self.repairing = true;
        ui.refresh();
        out
    }

    /// After a repair (`007b5b40`): closed as Cancel closes it (with
    /// `UIMenuMode`) when the item reached 99% or only it is left, else
    /// the lines again ([`RepairMenu::refill`]). Returns what the game
    /// should do.
    pub fn repaired(&mut self, ui: &mut Ui, input: repair::RepairInput) -> Vec<Action> {
        if !self.repairing {
            return Vec::new();
        }
        if RepairMenu::done_after(&input) {
            self.cancel_repair(ui)
        } else if let Some(r) = self.repair.as_mut() {
            let out = r.refill(ui, input);
            ui.refresh();
            out
        } else {
            Vec::new()
        }
    }

    /// Cancel (`007b5b40` case 0xc): `UIMenuMode`, back to ITEMS.
    pub fn cancel_repair(&mut self, ui: &mut Ui) -> Vec<Action> {
        self.close_repair(ui);
        vec![Action::Sound(repair::MENU_SOUND.into())]
    }

    fn close_repair(&mut self, ui: &mut Ui) {
        if !self.repairing {
            return;
        }
        self.repairing = false;
        self.let_pointer_go(ui);
        if let Some(r) = self.repair.as_mut() {
            r.hide(ui);
        }
        if self.section == Section::Items {
            ui.set_number(self.items.menu, t::VISIBLE, 1.0);
        }
        ui.refresh();
    }

    /// Opens the mod screen over ITEMS with what the game worked out (the
    /// first line chosen only with a pad). Returns what the game should do
    /// (the scroll knob).
    pub fn open_item_mod(&mut self, ui: &mut Ui, input: item_mod::ItemModInput) -> Vec<Action> {
        if self.item_mod.is_none() {
            return Vec::new();
        }
        self.let_pointer_go(ui);
        let pad = crate::game::has_pad(ui);
        ui.set_number(self.items.menu, t::VISIBLE, 0.0);
        let out = match self.item_mod.as_mut() {
            Some(m) => m.open(ui, input, pad),
            None => Vec::new(),
        };
        self.modding = true;
        ui.refresh();
        out
    }

    /// After a mod is fitted (`007838a0`): the list made again
    /// (`00784710(weapon, 0)`). Returns what the game should do.
    pub fn item_modded(&mut self, ui: &mut Ui, input: item_mod::ItemModInput) -> Vec<Action> {
        if !self.modding {
            return Vec::new();
        }
        let pad = crate::game::has_pad(ui);
        match self.item_mod.as_mut() {
            Some(m) => m.fill(ui, input, pad),
            None => Vec::new(),
        }
    }

    /// Cancel (`007838a0` case 0xc): `UIMenuMode`, back to ITEMS.
    pub fn cancel_item_mod(&mut self, ui: &mut Ui) -> Vec<Action> {
        self.close_item_mod(ui);
        vec![Action::Sound(item_mod::MENU_SOUND.into())]
    }

    fn close_item_mod(&mut self, ui: &mut Ui) {
        if !self.modding {
            return;
        }
        self.modding = false;
        self.let_pointer_go(ui);
        if let Some(m) = self.item_mod.as_mut() {
            m.hide(ui);
        }
        if self.section == Section::Items {
            ui.set_number(self.items.menu, t::VISIBLE, 1.0);
        }
        ui.refresh();
    }

    /// One frame of the menus' own movement (the repair screen's pulsing
    /// text), `now` in seconds.
    pub fn update(&mut self, ui: &mut Ui, now: f64) {
        if let Some(r) = self.repair.as_mut().filter(|_| self.repairing) {
            r.update(ui, now);
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tile::{Screen, SystemColors};

    pub fn ui() -> Ui {
        crate::game::new_ui(
            &mut |_| None,
            &|_: &str, _: &str| None,
            std::collections::HashMap::new(),
            1920,
            1080,
        )
    }

    #[test]
    fn tiles_by_id_ignore_ones_worked_out() {
        let mut ui = Ui::new(
            Screen {
                width_px: 1920,
                height_px: 1080,
                safe_x: 15.0,
                safe_y: 15.0,
            },
            SystemColors::new(None, None),
            Box::new(|_| None),
        );
        let m = ui
            .load_menu(
                b"<menu name=\"m\"><rect name=\"a\"><id>5</id></rect><rect name=\"b\"><id><copy>5</copy></id></rect>
                  <rect name=\"c\"><id>0</id></rect></menu>",
                &mut |_| None,
            )
            .unwrap();
        assert_eq!(by_id(&ui, m, 5), ui.find(m, "a"));
        assert_eq!(by_id(&ui, m, 0), ui.find(m, "c"));
        assert_eq!(by_id(&ui, m, 9), None);
    }

    /// `inventory_menu.xml` cut down, laid out for the pointer: rows 30
    /// high from y 100 (their `y` from `_y`, as `list_box_template.xml`
    /// places them), a tab line at y 600 of buttons 100 wide.
    const ITEMS: &str = r#"<menu name="InventoryMenu"><locus>&true;</locus>
      <hotrect name="list"><id>4</id><x>0</x><y>100</y><width>300</width><height>400</height><locus>&true;</locus>
        <target>&false;</target><wheelable>&true;</wheelable>
        <_scroll_delta>0</_scroll_delta><_highlight_y>-1</_highlight_y><_selected_height>0</_selected_height>
        <image name="lb_scrollbar"><_number_of_items>1</_number_of_items><_current_value>0</_current_value></image>
      </hotrect>
      <template name="IM_InventoryListTemplate"><hotrect name="row"><height>30</height><width>300</width>
        <y><copy src="me()" trait="_y"/></y><target>&true;</target></hotrect></template>
      <rect name="tabs"><id>13</id><x>0</x><y>600</y><width>855</width><locus>&true;</locus></rect>
      <template name="TabButtonTemplate"><hotrect name="TabButton"><width>100</width><height>30</height>
        <x><copy src="me()" trait="_x"/></x><mouseoversound>UIMenuFocus</mouseoversound></hotrect></template>
      <image name="IM_ItemIcon"><id>11</id><visible>&false;</visible></image>
      <rect name="IM_HotKeyWheel"><id>5</id><visible>&false;</visible><_SelectedHotkey>-1</_SelectedHotkey><_SelectedText></_SelectedText>
        <rect name="HK_Item_0"><_HotKeyAssigned>&false;</_HotKeyAssigned><_HotKeyIcon></_HotKeyIcon></rect>
        <rect name="HK_Item_2"><_HotKeyAssigned>&false;</_HotKeyAssigned><_HotKeyIcon></_HotKeyIcon></rect>
      </rect>
      <image name="IM_RepairButton"><id>8</id><x>900</x><width>10</width><height>10</height></image>
      <image name="IM_ModButton"><id>19</id><x>900</x><y>20</y><width>10</width><height>10</height></image>
    </menu>"#;

    /// `map_menu.xml` cut down: the world map in its clip window, the
    /// highlight box in its own, a marker template placed by `_x` / `_y`.
    const DATA: &str = r#"<menu name="MapMenu"><locus>&true;</locus>
      <hotrect name="MM_WorldMap_ClipWindow"><y>50</y><width>855</width><height>500</height><locus>&true;</locus><target>&false;</target>
        <hotrect name="world"><id>4</id><draggable>&true;</draggable><locus>&true;</locus><width>855</width><height>500</height>
          <_Magnification>1</_Magnification></hotrect>
      </hotrect>
      <hotrect name="MM_Highlight_ClipWindow"><y>50</y><width>855</width><height>500</height><locus>&true;</locus><target>&false;</target>
        <rect name="box"><id>6</id><width>160</width><height>160</height><x>348</x><y>170</y><locus>&true;</locus></rect>
      </hotrect>
      <template name="MapMarkerTemplate"><image name="MapMarker"><user0>1</user0><width>20</width><height>20</height><target>&false;</target>
        <x><copy src="me()" trait="_x"/><mul src="parent()" trait="width"/><sub>10</sub></x>
        <y><copy src="me()" trait="_y"/><mul src="parent()" trait="height"/><sub>10</sub></y></image></template>
    </menu>"#;

    fn load() -> (Ui, Pipboy) {
        let mut ui = ui();
        let mut read = |p: &str| {
            let text = match p {
                STATS_FILE => stats::tests::MENU,
                ITEMS_FILE => ITEMS,
                DATA_FILE => DATA,
                repair::FILE => repair::tests::MENU,
                item_mod::FILE => item_mod::tests::MENU,
                _ => return None,
            };
            Some(text.as_bytes().to_vec())
        };
        let p = Pipboy::load(&mut ui, &mut read).unwrap();
        (ui, p)
    }

    fn input() -> PipboyInput {
        let item = |form: u32, name: &str, tab: ItemTab| ItemLine {
            form,
            name: name.into(),
            count: 1,
            tab,
            equipped: false,
            usable: true,
            value: 10,
            weight: 1.0,
            icon: None,
            damage: None,
            dps: None,
            projectiles: 1,
            damage_resistance: None,
            damage_threshold: None,
            condition: None,
            strength: None,
            ammo: None,
            weight_class: None,
            effects: None,
            repairable: false,
            modded: false,
        };
        let marker = |form: u32, name: &str, at: [f32; 2]| MarkerLine {
            form,
            name: name.into(),
            at,
            kind: 1,
            travel: true,
        };
        PipboyInput {
            items: vec![
                item(0xA1, "Knife", ItemTab::Weapons),
                item(0xA2, "Pistol", ItemTab::Weapons),
                item(0xB1, "Vault Suit", ItemTab::Apparel),
            ],
            world_map: Some(WorldMapLine {
                picture: "map.dds".into(),
                size: [855.0, 500.0],
                markers: vec![
                    marker(0x901, "Goodsprings", [0.25, 0.75]),
                    marker(0x902, "Primm", [0.75, 0.25]),
                ],
                player: Some(([0.5, 0.5], 0.0)),
                corners: [[0.0, 1.0], [1.0, 0.0]],
                custom: None,
                quest: Vec::new(),
            }),
            ..stats::tests::input()
        }
    }

    const UP: Button = Button {
        down: false,
        pressed: false,
        released: false,
    };
    const PRESS: Button = Button {
        down: true,
        pressed: true,
        released: false,
    };
    const RELEASE: Button = Button {
        down: false,
        pressed: false,
        released: true,
    };

    fn sound(name: &str) -> Action {
        Action::Sound(name.into())
    }

    /// `0070c4a0` over ITEMS: the row under the pointer is chosen and its
    /// card shown (`00780ff0`, the knob clicking when the row changes),
    /// a press and release on it equips it (`00780140` case 0x1d), leaving
    /// it clears the choice (`00781620`), a tab button turns the tab
    /// (cases 0x18 .. 0x1c), and the pointer off the screen picks nothing.
    #[test]
    fn the_mouse_chooses_equips_and_turns_tabs_on_items() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Items);
        ui.refresh();
        // The second row (y 130 .. 160 under the list at 100).
        let out = p.pointer(&mut ui, Some([10.0, 145.0]), UP, 0.0, &input);
        // The knob a notch on: a later row than none.
        assert_eq!(
            out,
            [sound("UIPipBoyScroll"), Action::ScrollKnob { down: true }]
        );
        assert_eq!(p.items.list.selected, Some(1));
        // Up to the first row: the knob the other way.
        let out = p.pointer(&mut ui, Some([10.0, 115.0]), UP, 0.0, &input);
        assert_eq!(
            out,
            [sound("UIPipBoyScroll"), Action::ScrollKnob { down: false }]
        );
        p.pointer(&mut ui, Some([10.0, 145.0]), UP, 0.0, &input);
        // Pressed and let go over it: the pistol is equipped.
        p.pointer(&mut ui, Some([10.0, 145.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([10.0, 145.0]), RELEASE, 0.0, &input);
        assert_eq!(out, [Action::Equip(0xA2)]);
        // Off the screen: nothing under it, the choice let go.
        p.pointer(&mut ui, None, UP, 0.0, &input);
        assert_eq!(p.interface.over, None);
        assert_eq!(p.items.list.selected, None);
        // The same row again: no knob click (it was the last one).
        let out = p.pointer(&mut ui, Some([10.0, 150.0]), UP, 0.0, &input);
        assert!(out.is_empty());
        // The Apparel tab: gap trunc((855 - 500) / 6) = 59, so its button
        // spans 218 .. 318 along y 600 .. 630.
        // (Its `mouseoversound` from the file as the pointer comes onto it;
        // the knob, `00782470` → `007fa0f0`, as it's clicked.)
        let out = p.pointer(&mut ui, Some([250.0, 610.0]), UP, 0.0, &input);
        assert_eq!(out, [sound("UIMenuFocus")]);
        p.pointer(&mut ui, Some([250.0, 610.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([250.0, 610.0]), RELEASE, 0.0, &input);
        let apparel_help = Action::Tutorial {
            id: 0x23,
            menu: ITEMS_CLASS,
            delay: 512,
        };
        assert_eq!(out, [sound("UIPipBoyTab"), apparel_help.clone()]);
        assert_eq!(p.items.tab, 1);
        assert_eq!(p.items.shown, [0xB1]);
        // Pressed again: no turn, the help asked for all the same.
        p.pointer(&mut ui, Some([250.0, 610.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([250.0, 610.0]), RELEASE, 0.0, &input);
        assert_eq!(out, [apparel_help]);
    }

    /// The repair and mod screens, up over ITEMS, are the menu on top and
    /// take the pointer (their `DoEnter`, `DoLeave` and `DoClick`:
    /// `007b6120`, `007b6a00`, `007b5b40`; `00783ed0`, `00784050`,
    /// `007838a0`): the pointer's ITEMS tiles let go as one opens, a line
    /// under the pointer chosen with the knob's turn, a click on it mends
    /// or fits, the right button drops nothing, a click on Cancel goes back
    /// to ITEMS with `UIMenuMode`.
    #[test]
    fn the_repair_and_mod_screens_take_the_pointer() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Items);
        ui.refresh();
        p.pointer(&mut ui, Some([10.0, 145.0]), UP, 0.0, &input);
        assert!(p.interface.over.is_some());
        // No pad: nothing chosen as it opens, so no knob.
        assert!(p.open_repair(&mut ui, repair::tests::input()).is_empty());
        assert_eq!(p.interface.over, None);
        assert_eq!(p.menu(), p.repair.as_ref().unwrap().menu);
        ui.refresh();
        // The lines from y 75, 30 high: the third (0x20) at 135 .. 165.
        let out = p.pointer(&mut ui, Some([10.0, 150.0]), UP, 0.0, &input);
        assert_eq!(
            out,
            [sound("UIPipBoyScroll"), Action::ScrollKnob { down: true }]
        );
        assert_eq!(p.repair.as_ref().unwrap().list.selected, Some(2));
        p.pointer(&mut ui, Some([10.0, 150.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([10.0, 150.0]), RELEASE, 0.0, &input);
        assert_eq!(
            out,
            [
                Action::Repair {
                    chosen: 0x10,
                    part: 0x20
                },
                sound("UIRepairWeapon")
            ]
        );
        // ITEMS' every-frame code isn't on top: the right button drops
        // nothing.
        let out = p.pointer_with_right(&mut ui, Some([10.0, 150.0]), UP, PRESS, false, 0.0, &input);
        assert!(out.is_empty());
        // The wheel over the list: only its scroll bar reads it.
        assert!(p.wheel(&mut ui, -1, &input).is_empty());
        // Cancel, by the mouse.
        p.pointer(&mut ui, Some([920.0, 710.0]), UP, 0.0, &input);
        p.pointer(&mut ui, Some([920.0, 710.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([920.0, 710.0]), RELEASE, 0.0, &input);
        assert_eq!(out, [sound("UIMenuMode")]);
        assert!(!p.repairing);
        assert_eq!(ui.number(p.items.menu, t::VISIBLE), 1.0);
        assert_eq!(p.menu(), p.items.menu);

        // The mod screen: the second line (0x21, not fitted) under the
        // pointer at 130 .. 160 from y 100.
        assert!(p
            .open_item_mod(&mut ui, item_mod::tests::input())
            .is_empty());
        ui.refresh();
        let out = p.pointer(&mut ui, Some([10.0, 145.0]), UP, 0.0, &input);
        assert_eq!(
            out,
            [sound("UIPipBoyScroll"), Action::ScrollKnob { down: true }]
        );
        p.pointer(&mut ui, Some([10.0, 145.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([10.0, 145.0]), RELEASE, 0.0, &input);
        assert_eq!(
            out,
            [Action::FitMod {
                weapon: 0x10,
                item: 0x21
            }]
        );
        p.pointer(&mut ui, Some([920.0, 710.0]), UP, 0.0, &input);
        p.pointer(&mut ui, Some([920.0, 710.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([920.0, 710.0]), RELEASE, 0.0, &input);
        assert_eq!(out, [sound("UIMenuMode")]);
        assert!(!p.modding);
    }

    /// After a repair that brings the item to 99% the screen closes as
    /// Cancel closes it (`007b5b40`: click 0xc, `UIMenuMode`).
    #[test]
    fn a_repair_to_99_percent_closes_with_the_menu_sound() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Items);
        p.open_repair(&mut ui, repair::tests::input());
        let mut done = repair::tests::input();
        done.condition = 99.5;
        assert_eq!(p.repaired(&mut ui, done), [sound("UIMenuMode")]);
        assert!(!p.repairing);
    }

    /// `007f8720` / `0070c4a0`: the model's DATA button shows DATA with
    /// `UIMenuMode`, and the pointer's tiles of the old menu are let go.
    /// Over the world map (`0079a130`, `00799dc0`) the highlight box
    /// follows the pointer, the game's cursor hides, the nearest marker's
    /// name shows and a click on the map travels there.
    #[test]
    fn the_models_buttons_and_the_world_maps_markers() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.pointer(&mut ui, Some([10.0, 10.0]), UP, 0.0, &input);
        assert_eq!(p.press_section(&mut ui, 3), [sound("UIMenuMode")]);
        assert_eq!(p.section, Section::Data);
        assert_eq!(p.interface.over, None);
        ui.refresh();
        // Goodsprings is at (0.25 × 855, 0.75 × 500) on the map, which
        // sits at the clip window's corner (the player centred at 427.5,
        // 250), the window at y 50: (213.75, 425) on the screen.
        let out = p.pointer(&mut ui, Some([220.0, 420.0]), UP, 0.0, &input);
        assert_eq!(out, [sound("UIPipBoyHighlight")]);
        assert_eq!(p.data.marker, Some(0));
        assert!(p.cursor_hidden());
        let bx = by_id(&ui, p.data.menu, 6).unwrap();
        assert_eq!(
            (ui.number(bx, t::X), ui.number(bx, t::Y)),
            (220.0 - 80.0, 370.0 - 80.0)
        );
        let title = ui.names.lookup("_Title").unwrap();
        assert_eq!(ui.string(bx, title).unwrap(), "Goodsprings");
        p.pointer(&mut ui, Some([220.0, 420.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([220.0, 420.0]), RELEASE, 0.0, &input);
        assert_eq!(out, [Action::Travel(0x901)]);
        // Far from both markers: none; below the map the cursor shows.
        p.pointer(&mut ui, Some([427.0, 300.0]), UP, 0.0, &input);
        assert_eq!(p.data.marker, None);
        p.pointer(&mut ui, Some([427.0, 700.0]), UP, 0.0, &input);
        assert!(!p.cursor_hidden());
    }

    /// `00782a90` / `00780140` cases 0x1e, 10 and the tabs: with keys
    /// carried the Misc tab ends with the keyring's row; clicking it lists
    /// the keys; Cancel or another tab lists the tab's items again.
    #[test]
    fn the_keyring_opens_and_closes() {
        let (mut ui, mut p) = load();
        let mut input = input();
        let key = |form: u32, name: &str| ItemLine {
            form,
            name: name.into(),
            usable: false,
            ..input.items[0].clone()
        };
        input.keys = vec![key(0xC2, "Doc's Key"), key(0xC1, "Bunker Key")];
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Items);
        // Weapons: no keyring row.
        assert_eq!(p.items.shown, [0xA1, 0xA2]);
        p.items.show_tab(&mut ui, 3, &input);
        assert_eq!(p.items.shown, [0]);
        let row = p.items.list.rows[0];
        assert_eq!(ui.number(row, t::ID), items::KEYRING_ID as f32);
        p.click(&mut ui, items::KEYRING_ID, &input);
        assert!(p.items.keyring);
        assert_eq!(p.items.shown, [0xC1, 0xC2]);
        let open = ui.names.lookup("_KeyringOpen").unwrap();
        assert_eq!(ui.number(p.items.menu, open), 1.0);
        // A key can't be dropped from the keyring.
        assert_eq!(
            p.click(&mut ui, items::DROP_ID, &input),
            [sound("UIVATSInsufficientAP")]
        );
        p.click(&mut ui, items::CANCEL_ID, &input);
        assert!(!p.items.keyring);
        assert_eq!(p.items.shown, [0]);
        p.click(&mut ui, items::KEYRING_ID, &input);
        p.items.show_tab(&mut ui, 0, &input);
        assert!(!p.items.keyring);
        assert_eq!(ui.number(p.items.menu, open), 0.0);
    }

    /// `00781680`: with an item chosen Mod can be pressed and Repair only
    /// when the item can be repaired; with none chosen neither.
    #[test]
    fn items_buttons_follow_the_chosen_item() {
        let (mut ui, mut p) = load();
        let mut input = input();
        input.items[1].repairable = true;
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Items);
        ui.refresh();
        let target = |ui: &mut Ui, p: &Pipboy, id: i32| {
            ui.number(by_id(ui, p.items.menu, id).unwrap(), t::TARGET)
        };
        // The knife (first row, chosen on filling): Mod yes, Repair no.
        assert_eq!(
            (target(&mut ui, &p, 19), target(&mut ui, &p, 8)),
            (1.0, 0.0)
        );
        // The pistol under the pointer: both.
        p.pointer(&mut ui, Some([10.0, 145.0]), UP, 0.0, &input);
        assert_eq!(
            (target(&mut ui, &p, 19), target(&mut ui, &p, 8)),
            (1.0, 1.0)
        );
        // Off the rows: nothing chosen, neither.
        p.pointer(&mut ui, None, UP, 0.0, &input);
        assert_eq!(
            (target(&mut ui, &p, 19), target(&mut ui, &p, 8)),
            (0.0, 0.0)
        );
    }

    /// Dragging a map with the pointer leaving the screen: the map is not
    /// flung off by the point far away that "off the screen" is; it keeps
    /// the last point on the screen, and goes on from the pointer's return.
    #[test]
    fn a_drag_goes_on_when_the_pointer_leaves_the_screen() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Data);
        ui.refresh();
        let world = by_id(&ui, p.data.menu, 4).unwrap();
        let delta = |ui: &mut Ui| ui.number(world, crate::menu::drag::DELTA_X);
        const HELD: Button = Button {
            down: true,
            pressed: false,
            released: false,
        };
        p.pointer(&mut ui, Some([300.0, 300.0]), UP, 0.0, &input);
        p.pointer(&mut ui, Some([300.0, 300.0]), PRESS, 0.0, &input);
        assert!(p.interface.dragging.is_some());
        p.pointer(&mut ui, Some([280.0, 300.0]), HELD, 0.0, &input);
        assert_eq!(delta(&mut ui), -20.0);
        p.pointer(&mut ui, None, HELD, 0.0, &input);
        assert_eq!(delta(&mut ui), 0.0);
        assert!(p.interface.dragging.is_some());
        p.pointer(&mut ui, Some([260.0, 300.0]), HELD, 0.0, &input);
        assert_eq!(delta(&mut ui), -20.0);
        p.pointer(&mut ui, None, RELEASE, 0.0, &input);
        assert!(p.interface.dragging.is_none());
    }

    /// The right button going down (`00781ba0`, `0079a130`): on ITEMS it
    /// drops the chosen item (click 7) unless the Pip-Boy control is held
    /// or nothing is chosen; on the world map, over the map, it asks for
    /// the player's marker at the pointer's point of the picture.
    #[test]
    fn the_right_button_drops_and_marks_the_map() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Items);
        ui.refresh();
        let right = PRESS;
        // Over the second row: the pistol is chosen; Tab held: nothing.
        p.pointer(&mut ui, Some([10.0, 145.0]), UP, 0.0, &input);
        let held = p.pointer_with_right(&mut ui, Some([10.0, 145.0]), UP, right, true, 0.0, &input);
        assert!(!held.contains(&Action::Drop(0xA2)));
        let out = p.pointer_with_right(&mut ui, Some([10.0, 145.0]), UP, right, false, 0.0, &input);
        assert!(out.contains(&Action::Drop(0xA2)));
        // Off the rows nothing is chosen: nothing to drop.
        p.pointer(&mut ui, None, UP, 0.0, &input);
        let out = p.pointer_with_right(&mut ui, None, UP, right, false, 0.0, &input);
        assert!(!out.iter().any(|a| matches!(a, Action::Drop(_))));
        // DATA's world map: the pointer's point on the picture is (the
        // pointer less the map's place) ÷ the map's width, both ways.
        p.press_section(&mut ui, 3);
        ui.refresh();
        p.pointer(&mut ui, Some([220.0, 420.0]), UP, 0.0, &input);
        let world = by_id(&ui, p.data.menu, 4).unwrap();
        let (mx, my) = ui.screen_position(world);
        let w = ui.number(world, t::WIDTH);
        let out =
            p.pointer_with_right(&mut ui, Some([220.0, 420.0]), UP, right, false, 0.0, &input);
        assert!(out.contains(&Action::PlaceMarker([(220.0 - mx) / w, (420.0 - my) / w])));
        // Below the map (the cursor shows there): nothing.
        let out =
            p.pointer_with_right(&mut ui, Some([220.0, 700.0]), UP, right, false, 0.0, &input);
        assert!(!out.iter().any(|a| matches!(a, Action::PlaceMarker(_))));
    }

    /// `00781ba0`, `00780140` case 0x1d, `007019e0`: holding a number key on
    /// ITEMS shows the hot key wheel with that key highlighted (not the
    /// "2"); a row clicked then goes on that hot key; let go, the wheel
    /// hides. Ammunition can't go on one.
    #[test]
    fn number_keys_put_items_on_hot_keys() {
        let (mut ui, mut p) = load();
        let mut input = input();
        input.items.push(ItemLine {
            form: 0xC1,
            name: "9mm Round".into(),
            tab: ItemTab::Ammo,
            ..input.items[0].clone()
        });
        p.fill(&mut ui, &input);
        p.show(&mut ui, Section::Items);
        ui.refresh();
        let wheel = by_id(&ui, p.items.menu, 5).unwrap();
        let mut down = [false; 8];
        down[1] = true;
        p.hotkey_keys(&mut ui, down, &input);
        assert_eq!(ui.number(wheel, t::VISIBLE), 0.0);
        down = [false; 8];
        down[2] = true;
        p.hotkey_keys(&mut ui, down, &input);
        assert_eq!(ui.number(wheel, t::VISIBLE), 1.0);
        let selected = ui.names.lookup("_SelectedHotkey").unwrap();
        assert_eq!(ui.number(wheel, selected), 2.0);
        // The pistol (second row) clicked: on hot key 3.
        p.pointer(&mut ui, Some([10.0, 145.0]), UP, 0.0, &input);
        p.pointer(&mut ui, Some([10.0, 145.0]), PRESS, 0.0, &input);
        let out = p.pointer(&mut ui, Some([10.0, 145.0]), RELEASE, 0.0, &input);
        assert_eq!(
            out,
            [Action::SetHotkey {
                slot: 2,
                item: 0xA2
            }]
        );
        let place = ui.find_below(wheel, "HK_Item_2").unwrap();
        assert_eq!(ui.string(place, t::STRING).unwrap(), "Pistol");
        // Ammunition: refused.
        p.items.show_tab(&mut ui, 4, &input);
        ui.refresh();
        let out = p.click(&mut ui, items::ROW_ID, &input);
        assert!(out.contains(&Action::Sound("UIVATSInsufficientAP".into())));
        p.hotkey_keys(&mut ui, [false; 8], &input);
        assert_eq!(ui.number(wheel, t::VISIBLE), 0.0);
    }

    /// `007dbfa0` → `007dc040`: the limb healing mode aims at blinks, its
    /// alpha from 255 to 0 and back each second; leaving it stops at 255.
    #[test]
    fn the_aimed_limb_blinks() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.stats.healing = true;
        p.stats.part = Some(1);
        p.frame(&mut ui, 10.0);
        let limb = stats::limb_tiles(&ui, p.stats.menu, 1)[0];
        p.frame(&mut ui, 10.5);
        assert_eq!(ui.number(limb, t::ALPHA), 0.0);
        p.frame(&mut ui, 10.75);
        assert_eq!(ui.number(limb, t::ALPHA), 127.5);
        p.stats.healing = false;
        p.frame(&mut ui, 11.0);
        assert_eq!(ui.number(limb, t::ALPHA), 255.0);
    }

    /// `007db380` → `007e0060`: the Status page's RAD button (0x1d) makes
    /// RAD the shown mode; a mode past EFF needs hardcore.
    #[test]
    fn the_status_pages_mode_buttons() {
        let (mut ui, mut p) = load();
        let input = input();
        p.fill(&mut ui, &input);
        p.click(&mut ui, 0x1d, &input);
        assert_eq!(p.stats.status_mode, 1);
        p.click(&mut ui, 0x35, &input);
        assert_eq!(p.stats.status_mode, 1);
    }
}

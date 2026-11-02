//! Containers (`menus\container_menu.xml`, class `ContainerMenu` 1008,
//! vtable `010721ac` in FalloutNV.exe): the player's things on the left,
//! the container's on the right. The menu also serves pickpocketing (mode
//! 2), a companion's trade (3) and a fourth use (4); only mode 1, a
//! container or a body, is here. Read from the code:
//!
//! * opening (`0075b310`): the two list boxes (ids 4 and 8, template
//!   `CM_list_template`); `CM_ItemsTitle` `sInventoryItems`, Take All
//!   `sTakeAll`, Exit `sExit`; the container's name fitted into
//!   `CM_ContainerTitle` (`0075e8c0`: wider than the title's frame less 80,
//!   and 35 more with the left arrow shown, it's cut to the characters that
//!   fit with "..." added); its opening sound (`0075baf0`: a container's
//!   `SNAM`, a body's `DRSBodyGenericOpen`); then filled;
//! * filling (`0075c280`): each side's items (`00719ef0`), leaving out
//!   (`0075cfc0`) things without a name and armour, weapons and ammunition
//!   marked not playable; an item's text its name (+ "+" for a weapon with
//!   mods); then (`0075d160`) "name (count)" for more than one and the
//!   worn mark (`CM_list_template_ItemMarker`) on what's equipped; the
//!   lists filtered and sorted by their text (`_mbscmp`); the weight line
//!   (`CM_Items_CapsLabel`) "`sInventoryWeight`   inventory/carry";
//! * the filters (`0075e650`), one per side, 0 all, 1 weapons, 2 apparel
//!   (armour, clothing), 3 aid (ingredients, aid, books), 4 misc (all
//!   but those, ammunition, chips, caravan cards and money), 5 ammunition;
//!   ammunition in the `RegeneratingAmmo` list never shows; the player's
//!   side never shows keys or quest items. The arrows and the title step
//!   the filter (`0075be80`, the title's wheel `0075d4a0`), on past
//!   filters with nothing to show, back round to 0; titles (`0075e380`)
//!   the filter's setting, upper case when `[General] sLanguage` is
//!   "ENGLISH" (filter 0: the player's `sInventoryItems`, the container's
//!   fitted name);
//! * the pointer on an item (`0075cd70`, id 20) makes its side the current
//!   one and shows its picture (`CM_ItemIcon`: the item's `ICON`, else
//!   `sMissingImage`) and the arrow (`CM_ArrowIcon` `user0` 1 giving, 2
//!   taking); off it (`0075cf70`) they hide;
//! * a click on an item (`0075be80` case 0x14): caps and weightless things
//!   taken from the container go whole; else more than
//!   `iInventoryAskQuantityAt` (5) asks how many (`QuantityMenu`), else
//!   one moves (`0075dc80`, `0075d6b0`): the item's pick-up sound, the
//!   giver's `RemoveItem` into the other, the lists refreshed;
//! * Take All (id 10): every item the container's list shows, whole, then
//!   `UIItemTakeAll` and the menu closes; Exit (id 11) and the space bar
//!   (`0075c250`) close it (`0075b750`: the closing sound, a container's
//!   `QNAM`, a body's `DRSBodyGenericClose`);
//! * Left and Right (`0075d250`) step the current side's filter;
//! * after a change (`0075c280`) a current side left with nothing shown
//!   steps its filter on (silently) and, still empty, gives way to the
//!   other side.
//!
//! Not here: the item card (`CM_ItemData`, `00707e30`, shared with the
//! Pip-Boy's inventory; it stays hidden), armed mines' "Live" lines, a
//! dead person's held weapon added to the list, items' extra data (their
//! owners, the pick-up reference test), modes 2 and 4, the controller's
//! buttons.
//!
//! The subtitle (`CM_Subtitle`, found by name): emptied on opening
//! (`0075b310`); a companion's line in mode 3 (their `FollowersTrade` and
//! `FollowersOverburdened` barks, `0075ec60` → `0075eea0`) is put there
//! until its voice's length, or its length × `fNoticeTextTimePerCharacter`
//! without a voice, has passed (`+0xfc`); each frame (`0075eac0`) empties
//! it after ([`ContainerMenu::say`], [`ContainerMenu::update`]).

use crate::list::{font_for, ListBox};
use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::text;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\container_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1008;
/// The template its lists' items are made from.
pub const LIST_TEMPLATE: &str = "CM_list_template";
/// `iInventoryAskQuantityAt`'s default: more than this asks how many.
pub const ASK_QUANTITY_AT: i32 = 5;
/// Take All's sound.
pub const TAKE_ALL_SOUND: &str = "UIItemTakeAll";
/// The id an item of either list has (`CM_list_template_container`).
pub const ITEM_ID: i32 = 20;
/// How many tile ids the menu needs (`0075b710`: 0 to 18, or it fails).
pub const TILE_COUNT: usize = 19;

/// The game's form type numbers the filters test.
pub mod form_type {
    pub const ARMO: u8 = 0x18;
    pub const BOOK: u8 = 0x19;
    pub const CLOT: u8 = 0x1A;
    pub const INGR: u8 = 0x1D;
    pub const WEAP: u8 = 0x28;
    pub const AMMO: u8 = 0x29;
    pub const KEYM: u8 = 0x2E;
    pub const ALCH: u8 = 0x2F;
    pub const CHIP: u8 = 0x6C;
    pub const CCRD: u8 = 0x73;
    pub const CMNY: u8 = 0x74;
}

/// One thing a side holds, with what the menu reads of it.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Item {
    /// The caller's handle (the item's form ID).
    pub form: u32,
    /// Its name (`TESFullName`); without one it isn't listed.
    pub name: String,
    pub count: i32,
    /// The game's form type number (`form_type`).
    pub form_type: u8,
    /// Worn or held by the holder (`004bddd0`).
    pub equipped: bool,
    /// Form flag 0x400 (`vfunc +0x94`).
    pub quest_item: bool,
    /// Armour, clothing, weapons and ammunition can be marked not
    /// playable (`BMDT` 0x40, `DNAM` flags 0x80, `DATA` flags 0x02).
    pub playable: bool,
    /// Ammunition in the `RegeneratingAmmo` form list.
    pub regenerating_ammo: bool,
    /// A weapon with mods on it (`ExtraWeaponModFlags`).
    pub modded: bool,
    /// Caps (form `0000000F`) or weighing nothing: taken whole.
    pub weightless: bool,
    /// Caps themselves (`00481f10`: form `0000000F`).
    pub is_caps: bool,
    /// The picture for `CM_ItemIcon` (its `ICON`; empty: `sMissingImage`).
    pub icon: String,
}

/// The two sides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// The player's things (the list at `+0x98`, id 4).
    Player,
    /// The container's (`+0xc8`, id 8).
    Container,
}

impl Side {
    pub fn other(self) -> Side {
        match self {
            Side::Player => Side::Container,
            Side::Container => Side::Player,
        }
    }
}

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// Move `count` of an item from `from` to the other side.
    Move { from: Side, form: u32, count: i32 },
    /// Ask how many to move (`007aba00` with `count` the most).
    AskQuantity { from: Side, form: u32, most: i32 },
    /// Close: run the container's closing (sound, scripts).
    Close,
}

/// The container menu.
#[derive(Debug)]
pub struct ContainerMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..18 at `+0x28` (`0075b210`).
    pub tiles: [Option<TileId>; TILE_COUNT],
    pub lists: [ListBox; 2],
    pub items: [Vec<Item>; 2],
    /// Each side's filter (`+0x8c`, `+0x90`).
    pub filters: [i32; 2],
    /// The side the pointer last chose an item on (`+0xf8`).
    pub current: Option<Side>,
    /// The item under the pointer (`011d93fc`): side and index.
    pub selected: Option<(Side, usize)>,
    /// The container's name, for its title.
    pub name: String,
    /// `[General] sLanguage` is "ENGLISH": titles in upper case.
    pub english: bool,
    pub ask_quantity_at: i32,
    /// The weight line's two numbers: inventory and carry weight.
    pub weights: (f32, f32),
    /// The filter-step sound's number (`+0x94`: 3, -1 while stepping on
    /// by itself).
    filter_sound: i32,
    /// What it's for (`+0x80`): 1 a container or a body, 3 a companion's
    /// things (`OpenTeammateContainer`); 2 and 4 aren't here.
    pub mode: i32,
    /// What the menu asks of the game, and sounds to play (editor IDs).
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
    /// When the subtitle empties, in ms (`+0xfc`).
    pub subtitle_until: Option<f64>,
}

fn side_index(side: Side) -> usize {
    match side {
        Side::Player => 0,
        Side::Container => 1,
    }
}

/// Whether the filter hides an item (`0075e650`).
pub fn hidden(item: &Item, side: Side, filter: i32) -> bool {
    use form_type::*;
    let ty = item.form_type;
    let mut out = match filter {
        1 => ty != WEAP,
        2 => !(ty == ARMO || ty == CLOT),
        3 => !(ty == INGR || ty == ALCH || ty == BOOK),
        4 => [WEAP, ARMO, CLOT, INGR, ALCH, AMMO, BOOK, CHIP, CCRD, CMNY].contains(&ty),
        5 => ty != AMMO,
        _ => false,
    };
    if ty == AMMO && (item.regenerating_ammo || !item.playable) {
        out = true;
    }
    match side {
        Side::Player => out || ty == KEYM || item.quest_item,
        Side::Container => out,
    }
}

/// Whether an item is listed at all (`0075cfc0` answers the opposite): it
/// has a name, and isn't marked not playable.
pub fn listed(item: &Item) -> bool {
    !item.name.is_empty() && item.playable && item.count > 0
}

/// The text an item is added with (`00719ef0`): its name, "+" after a
/// weapon with mods.
fn first_text(item: &Item) -> String {
    if item.modded {
        format!("{}+", item.name)
    } else {
        item.name.clone()
    }
}

/// Rounds as `004bd510` does (to the nearest, halves away from the whole
/// part).
fn round(v: f32) -> f32 {
    v.trunc()
        + if (v - v.trunc()).abs() >= 0.5 {
            v.signum()
        } else {
            0.0
        }
}

impl ContainerMenu {
    pub fn new(menu: TileId) -> ContainerMenu {
        ContainerMenu {
            menu,
            tiles: [None; TILE_COUNT],
            lists: [ListBox::default(), ListBox::default()],
            items: [Vec::new(), Vec::new()],
            filters: [0, 0],
            current: None,
            selected: None,
            name: String::new(),
            english: true,
            ask_quantity_at: ASK_QUANTITY_AT,
            weights: (0.0, 0.0),
            filter_sound: 3,
            mode: 1,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
            subtitle_until: None,
        }
    }

    /// A line said (`0075eea0`): on `CM_Subtitle` until `until` (ms).
    pub fn say(&mut self, ui: &mut Ui, text: &str, until: f64) {
        if let Some(sub) = ui.find(self.menu, "CM_Subtitle") {
            ui.set_text(sub, t::STRING, text);
        }
        self.subtitle_until = Some(until);
        ui.refresh();
    }

    /// Each frame (`0075eac0`): the subtitle emptied once its time is up.
    pub fn update(&mut self, ui: &mut Ui, now: f64) {
        if self.subtitle_until.is_some_and(|until| now > until) {
            self.subtitle_until = None;
            if let Some(sub) = ui.find(self.menu, "CM_Subtitle") {
                ui.set_text(sub, t::STRING, "");
            }
            ui.refresh();
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    fn list_tile(side: Side) -> usize {
        match side {
            Side::Player => 4,
            Side::Container => 8,
        }
    }

    /// Opens it (`0075b310`, mode 1): false when the file lacks one of the
    /// menu's tiles ("Container Menu Creation Failed").
    pub fn open(&mut self, ui: &mut Ui, player: Vec<Item>, container: Vec<Item>) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        for side in [Side::Player, Side::Container] {
            if let Some(list) = self.tile(Self::list_tile(side)) {
                self.lists[side_index(side)] = ListBox::new(ui, list, LIST_TEMPLATE);
            }
        }
        let text = |ui: &Ui, name: &str| ui.setting_text(name).unwrap_or_default();
        for (id, setting) in [(1, "sInventoryItems"), (10, "sTakeAll"), (11, "sExit")] {
            if let Some(tile) = self.tile(id) {
                let s = text(ui, setting);
                ui.set_text(tile, t::STRING, &s);
            }
        }
        if let Some(sub) = ui.find(self.menu, "CM_Subtitle") {
            ui.set_text(sub, t::STRING, "");
        }
        // A companion's things (`0075bc80` mode 3): no Take All.
        if self.mode == 3 {
            if let Some(take_all) = self.tile(10) {
                ui.set_number(take_all, t::VISIBLE, 0.0);
            }
        }
        self.fit_name(ui);
        // The item card's titles (`_Title` on ids 14..18).
        let title = ui.names.lookup_or_add("_Title").unwrap_or(0);
        for (id, setting) in [
            (14, "sInventoryDamage"),
            (15, "sInventoryDamagePerSecond"),
            (16, "sInventoryStrReq"),
            (17, "sInventoryDamageResistance"),
            (18, "sInventoryDamageThreshold"),
        ] {
            if let Some(tile) = self.tile(id) {
                let s = text(ui, setting);
                ui.set_string(tile, title, &s);
            }
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        self.items = [player, container];
        self.fill(ui);
        true
    }

    /// The container's name in its title (`0075e8c0`).
    fn fit_name(&mut self, ui: &mut Ui) {
        let Some(title) = self.tile(6) else {
            return;
        };
        if self.name.is_empty() {
            return;
        }
        let font_index = ui.number(title, t::FONT) as i32;
        let Some(font) = font_for(ui, font_index) else {
            return;
        };
        let width = text::measure(
            &font,
            font_index as usize,
            self.name.as_bytes(),
            f32::MAX,
            0,
        )
        .width;
        let parent_width = ui.tiles[title]
            .parent
            .map_or(0.0, |p| ui.number(p, t::WIDTH));
        let mut limit = parent_width - 80.0;
        if let Some(left) = self.tile(5) {
            if ui.number(left, t::VISIBLE) != 0.0 {
                limit -= 35.0;
            }
        }
        let advance = |c: u8| {
            let g = font.glyph(c);
            g.kern_left + g.width + g.kern_right
        };
        if width <= limit {
            let name = self.name.clone();
            ui.set_text(title, t::STRING, &name);
            return;
        }
        // The dots first, then characters while the width so far fits
        // (the last one taken may pass it).
        let bytes = self.name.as_bytes();
        let mut w = 3.0 * advance(b'.');
        let mut cut = Vec::new();
        let mut i = 0;
        while w <= limit && i < 0x104 && i < bytes.len() {
            cut.push(bytes[i]);
            w += advance(bytes[i]);
            i += 1;
        }
        cut.extend_from_slice(b"...");
        let s = String::from_utf8_lossy(&cut).into_owned();
        ui.set_text(title, t::STRING, &s);
    }

    /// A side's title for its filter (`0075e380`).
    fn title_for_filter(&mut self, ui: &mut Ui, side: Side) {
        let filter = self.filters[side_index(side)];
        let tile = match side {
            Side::Player => self.tile(1),
            Side::Container => self.tile(6),
        };
        let Some(tile) = tile else {
            return;
        };
        let setting = match filter {
            0 if side == Side::Container => {
                self.fit_name(ui);
                return;
            }
            0 => "sInventoryItems",
            1 => "sInventoryWeapons",
            2 => "sInventoryApparel",
            3 => "sInventoryAid",
            4 => "sInventoryMisc",
            5 => "sInventoryAmmo",
            _ => return,
        };
        let mut s = ui.setting_text(setting).unwrap_or_default();
        if self.english {
            s = s.to_ascii_uppercase();
        }
        ui.set_text(tile, t::STRING, &s);
    }

    /// Fills both lists from the items (`0075c280` with no item: each list
    /// emptied and filled again), then the labels, filters, order and the
    /// weight line.
    pub fn fill(&mut self, ui: &mut Ui) {
        for side in [Side::Player, Side::Container] {
            let s = side_index(side);
            self.lists[s].clear(ui);
            for i in 0..self.items[s].len() {
                let item = &self.items[s][i];
                if !listed(item) {
                    continue;
                }
                let text = first_text(item);
                self.lists[s].add(ui, self.menu, i as i32, Some(&text));
            }
        }
        self.finish(ui);
    }

    /// One item changed (`0075c280` with an item, after a move): its lines
    /// taken out of both lists and added again (at the end) from the new
    /// `items`, then the rest as for [`ContainerMenu::fill`].
    pub fn refresh_item(
        &mut self,
        ui: &mut Ui,
        form: u32,
        player: Vec<Item>,
        container: Vec<Item>,
    ) {
        // The current side's chosen line and scroll position, kept
        // (`007312e0`).
        let kept = self.current.map(|side| self.save_place(ui, side));
        let old = std::mem::replace(&mut self.items, [player, container]);
        for side in [Side::Player, Side::Container] {
            let s = side_index(side);
            // Lines of other items keep their tiles; their indexes are
            // matched up with the new items by form.
            let lines: Vec<_> = self.lists[s].items.clone();
            for line in lines {
                let Some(old_item) = old[s].get(line.value as usize) else {
                    continue;
                };
                let new_index = self.items[s].iter().position(|i| i.form == old_item.form);
                match new_index {
                    Some(n) if old_item.form != form => {
                        if let Some(l) =
                            self.lists[s].items.iter_mut().find(|l| l.tile == line.tile)
                        {
                            l.value = n as i32;
                        }
                    }
                    _ => self.lists[s].remove(ui, line.tile),
                }
            }
            for (i, item) in self.items[s].iter().enumerate() {
                if item.form == form && listed(item) {
                    let text = first_text(item);
                    self.lists[s].add(ui, self.menu, i as i32, Some(&text));
                }
            }
        }
        if let Some((side, place)) = self.selected {
            let gone = old[side_index(side)]
                .get(place)
                .map_or(true, |i| i.form == form);
            if gone {
                self.selected = None;
            } else {
                let f = old[side_index(side)][place].form;
                self.selected = self.items[side_index(side)]
                    .iter()
                    .position(|i| i.form == f)
                    .map(|n| (side, n));
            }
        }
        self.finish(ui);
        if let (Some(side), Some(place)) = (self.current, kept) {
            self.after_change(ui, side, place);
        }
    }

    /// The labels, filters, order and weight line (the end of `0075c280`).
    fn finish(&mut self, ui: &mut Ui) {
        for side in [Side::Player, Side::Container] {
            let s = side_index(side);
            let lines = self.lists[s].shown_items(ui, 0, i32::MAX);
            for line in lines {
                if let Some(item) = self.items[s].get(line.value as usize).cloned() {
                    self.label(ui, line.tile, &item);
                }
            }
            let filter = self.filters[s];
            let items = self.items[s].clone();
            self.lists[s].filter(ui, &|v| {
                items
                    .get(v as usize)
                    .map_or(true, |item| hidden(item, side, filter))
            });
            self.lists[s].sort(ui, &|ui, a, b| {
                let sa = ui.string(a, t::STRING).unwrap_or_default();
                let sb = ui.string(b, t::STRING).unwrap_or_default();
                sa.as_bytes() < sb.as_bytes()
            });
        }
        self.weight_line(ui);
        ui.refresh();
    }

    /// An item's line (`0075d160`): the worn mark, "name (count)", the
    /// text copied to `ListItemText`, its height worked out again.
    fn label(&mut self, ui: &mut Ui, tile: TileId, item: &Item) {
        if item.equipped {
            if let Some(mark) = ui.find_below(tile, "CM_list_template_ItemMarker") {
                ui.set_number(mark, t::VISIBLE, 1.0);
            }
        }
        if item.count > 1 {
            let s = format!("{} ({})", item.name, item.count);
            ui.set_string(tile, t::STRING, &s);
        }
        let s = ui.string(tile, t::STRING).unwrap_or_default();
        if let Some(text_tile) = ui.find_below(tile, "ListItemText") {
            ui.set_text(text_tile, t::STRING, &s);
        }
        ui.set_number(tile, t::HEIGHT, 0.0);
    }

    /// "`sInventoryWeight`   inventory/carry", each rounded.
    fn weight_line(&mut self, ui: &mut Ui) {
        let Some(label) = self.tile(3) else {
            return;
        };
        let wg = ui.setting_text("sInventoryWeight").unwrap_or_default();
        let s = format!(
            "{wg}   {}/{}",
            round(self.weights.0) as i32,
            round(self.weights.1) as i32
        );
        ui.set_text(label, t::STRING, &s);
    }

    /// The current side's chosen line and scroll position (`007312e0`).
    fn save_place(&self, ui: &mut Ui, side: Side) -> (f32, f32) {
        let list = &self.lists[side_index(side)];
        let index = list.selected.map_or(-1.0, |s| ui.number(s, t::LISTINDEX));
        let current = ui.names.lookup_or_add("_current_value").unwrap_or(0);
        let scroll = list.scrollbar.map_or(-1.0, |b| ui.number(b, current));
        (index, scroll)
    }

    /// After a change (the end of `0075c280`): a current side with nothing
    /// shown steps its filter on silently, and still empty gives way to
    /// the other side; else its scroll position comes back (`00731360`:
    /// with the mouse the chosen line is let go).
    fn after_change(&mut self, ui: &mut Ui, side: Side, place: (f32, f32)) {
        let s = side_index(side);
        if self.lists[s].shown_count(ui) == 0 {
            self.filter_sound = -1;
            let id = match side {
                Side::Player => 2,
                Side::Container => 7,
            };
            self.click(ui, id, None, 0.0);
            self.filter_sound = 3;
            if self.lists[s].shown_count(ui) == 0 {
                self.switch_to(ui, side.other());
            }
            return;
        }
        self.restore_place(ui, side, place);
    }

    /// The scroll position back (`00731360`): with the mouse, the line
    /// chosen at the old place is let go at once (and its item no longer
    /// shown).
    fn restore_place(&mut self, ui: &mut Ui, side: Side, place: (f32, f32)) {
        let list = &mut self.lists[side_index(side)];
        if let Some(bar) = list.scrollbar {
            let current = ui.names.lookup_or_add("_current_value").unwrap_or(0);
            crate::list::set_keeping_operators(ui, bar, current, place.1);
            let set_in_code = ui.names.lookup_or_add("_SetInCode").unwrap_or(0);
            ui.set_number(bar, set_in_code, 1.0);
        }
        let at = list.item_at(ui, place.0 as i32);
        list.select(ui, None);
        if at.is_some() {
            self.unmouseover(ui, ITEM_ID, self.menu);
        }
    }

    /// Steps a side's filter (`0075be80` cases 0–2, 5–7): on in `dir`
    /// (1 or -1), wrapping 0..5, past filters with nothing to show, until
    /// back at 0.
    fn step_filter(&mut self, ui: &mut Ui, side: Side, dir: i32) {
        if let Some(s) = menu::menu_sound(self.filter_sound) {
            self.sounds.push(s.to_string());
        }
        let s = side_index(side);
        loop {
            let mut f = self.filters[s] + dir;
            if f < 0 {
                f = 5;
            } else if f > 5 {
                f = 0;
            }
            self.filters[s] = f;
            self.title_for_filter(ui, side);
            let items = self.items[s].clone();
            self.lists[s].filter(ui, &|v| {
                items
                    .get(v as usize)
                    .map_or(true, |item| hidden(item, side, f))
            });
            if self.lists[s].shown_count(ui) != 0 || self.filters[s] == 0 {
                break;
            }
        }
        ui.refresh();
    }

    /// The pointer's side changes (`0075cd70` cases 0x15, 0x16): the item
    /// shown goes, the other list lets its line go.
    fn switch_to(&mut self, ui: &mut Ui, side: Side) {
        self.unmouseover(ui, ITEM_ID, self.menu);
        let other = side_index(side.other());
        self.lists[other].select(ui, None);
        self.current = Some(side);
    }

    /// Whether an item can be moved: its line found, and its text not drawn
    /// half see-through (alpha 128, as the game marks armed mines' lines;
    /// `0075dc80`).
    fn movable(&self, ui: &mut Ui, side: Side, index: usize) -> bool {
        let list = &self.lists[side_index(side)];
        let Some(line) = list.items.iter().find(|l| l.value == index as i32) else {
            return false;
        };
        ui.find_below(line.tile, "ListItemText")
            .map_or(true, |text| ui.number(text, t::ALPHA) != 128.0)
    }

    /// The answer of "how many" (`0075dc80` as the quantity menu's
    /// callback): that many of the chosen item move; 0 nothing.
    pub fn quantity_chosen(&mut self, ui: &mut Ui, count: i32) {
        if count <= 0 {
            return;
        }
        if let Some((side, index)) = self.selected {
            if self.movable(ui, side, index) {
                if let Some(item) = self.items[side_index(side)].get(index) {
                    self.requests.push(Request::Move {
                        from: side,
                        form: item.form,
                        count,
                    });
                }
            }
        }
    }

    /// Closes it (`0075b750`).
    pub fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        self.requests.push(Request::Close);
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }
}

impl MenuCode for ContainerMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..TILE_COUNT as i32).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `0075be80`.
    fn click(&mut self, ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        match id {
            0..=2 => self.step_filter(ui, Side::Player, if id == 0 { -1 } else { 1 }),
            5..=7 => self.step_filter(ui, Side::Container, if id == 5 { -1 } else { 1 }),
            10 => {
                // Every line the container's list shows, whole.
                let lines = self.lists[1].shown_items(ui, 0, i32::MAX);
                for line in lines {
                    if !self.movable(ui, Side::Container, line.value as usize) {
                        continue;
                    }
                    if let Some(item) = self.items[1].get(line.value as usize) {
                        self.requests.push(Request::Move {
                            from: Side::Container,
                            form: item.form,
                            count: item.count,
                        });
                    }
                }
                self.sounds.push(TAKE_ALL_SOUND.to_string());
                self.close(ui);
            }
            11 => self.close(ui),
            ITEM_ID => {
                let Some((side, index)) = self.selected else {
                    return;
                };
                let Some(item) = self.items[side_index(side)].get(index).cloned() else {
                    return;
                };
                let whole = item.weightless && self.current == Some(Side::Container);
                if item.count > self.ask_quantity_at && !whole {
                    if let Some(c) = self.current {
                        self.lists[side_index(c)].select(ui, None);
                    }
                    self.requests.push(Request::AskQuantity {
                        from: side,
                        form: item.form,
                        most: item.count,
                    });
                } else if self.movable(ui, side, index) {
                    self.requests.push(Request::Move {
                        from: side,
                        form: item.form,
                        count: if whole { item.count } else { 1 },
                    });
                }
            }
            _ => {}
        }
    }

    /// `0075cd70`.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if id != ITEM_ID {
            return;
        }
        let parent = ui.tiles[tile].parent;
        let on = [Side::Player, Side::Container]
            .into_iter()
            .find(|&s| parent.is_some() && parent == self.tile(Self::list_tile(s)));
        if let Some(side) = on {
            if self.current != Some(side) {
                self.switch_to(ui, side);
            }
        }
        let Some(side) = self.current else {
            return;
        };
        let list = &self.lists[side_index(side)];
        self.selected = list
            .selected
            .and_then(|s| list.value_of(s))
            .map(|v| (side, v as usize));
        let Some((side, index)) = self.selected else {
            return;
        };
        let Some(item) = self.items[side_index(side)].get(index).cloned() else {
            return;
        };
        if let Some(icon) = self.tile(12) {
            let path = if item.icon.is_empty() {
                ui.setting_text("sMissingImage").unwrap_or_default()
            } else {
                item.icon.clone()
            };
            ui.set_string(icon, t::FILENAME, &path);
            ui.set_number(icon, t::VISIBLE, 1.0);
        }
        if let Some(arrow) = self.tile(9) {
            let way = if side == Side::Player { 1.0 } else { 2.0 };
            ui.set_number(arrow, t::USER0, way);
            ui.set_number(arrow, t::VISIBLE, 1.0);
        }
        ui.refresh();
    }

    /// `0075cf70`: the card, picture and arrow hidden.
    fn unmouseover(&mut self, ui: &mut Ui, _id: i32, _tile: TileId) {
        for id in [13, 12, 9] {
            if let Some(tile) = self.tile(id) {
                ui.set_number(tile, t::VISIBLE, 0.0);
            }
        }
        ui.refresh();
    }

    /// `0075c250`: the space bar closes it.
    fn key(&mut self, ui: &mut Ui, code: u32, _now: f64) -> bool {
        if code == 0x20 {
            self.close(ui);
            return true;
        }
        false
    }

    /// `0075d250`: Left and Right step the current side's filter.
    fn special_key(&mut self, ui: &mut Ui, code: i32, now: f64) -> bool {
        if code != menu::special::LEFT && code != menu::special::RIGHT {
            return false;
        }
        let left = code == menu::special::LEFT;
        let side = self.current.unwrap_or(Side::Container);
        let place = self.save_place(ui, side);
        let id = match (side, left) {
            (Side::Player, true) => 0,
            (Side::Player, false) => 2,
            (Side::Container, true) => 5,
            (Side::Container, false) => 7,
        };
        self.click(ui, id, None, now);
        self.restore_place(ui, side, place);
        true
    }

    /// `0075d4a0`: the wheel over a title steps its filter (down: back).
    fn wheel(&mut self, ui: &mut Ui, id: i32, _tile: TileId, delta: i32) {
        let first = match id {
            1 => 0,
            6 => 5,
            _ => return,
        };
        let id = if delta < 0 { first } else { first + 2 };
        self.click(ui, id, None, 0.0);
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        self.lists.iter_mut().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn item(form: u32, name: &str, count: i32, ty: u8) -> Item {
        Item {
            form,
            name: name.into(),
            count,
            form_type: ty,
            playable: true,
            ..Item::default()
        }
    }

    fn opened(player: Vec<Item>, container: Vec<Item>, name: &str) -> (Ui, ContainerMenu) {
        let mut ui = test_support::ui();
        let mut m = ContainerMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::container_menu(), &mut m);
        m.name = name.into();
        m.weights = (12.5, 199.4);
        assert!(m.open(&mut ui, player, container));
        (ui, m)
    }

    /// The text of a side's lines, in order, shown ones only.
    fn texts(ui: &mut Ui, m: &ContainerMenu, side: Side) -> Vec<String> {
        m.lists[side_index(side)]
            .shown_items(ui, 0, i32::MAX)
            .iter()
            .map(|l| ui.string(l.tile, t::STRING).unwrap_or_default())
            .collect()
    }

    /// `0075e650`: the filters by form type; ammunition in the regenerating
    /// list never shows; the player's keys and quest items never show.
    #[test]
    fn what_each_filter_shows() {
        use form_type::*;
        let gun = item(1, "Gun", 1, WEAP);
        let suit = item(2, "Suit", 1, ARMO);
        let book = item(3, "Book", 1, BOOK);
        let key = item(4, "Key", 1, KEYM);
        let ammo = item(5, "Rounds", 9, AMMO);
        let chip = item(6, "Chip", 1, CHIP);
        assert!(!hidden(&gun, Side::Container, 1));
        assert!(hidden(&suit, Side::Container, 1));
        assert!(!hidden(&suit, Side::Container, 2));
        assert!(!hidden(&book, Side::Container, 3));
        assert!(!hidden(&key, Side::Container, 4));
        assert!(hidden(&chip, Side::Container, 4));
        assert!(!hidden(&chip, Side::Container, 0));
        assert!(!hidden(&ammo, Side::Container, 5));
        // Keys and quest items stay off the player's side.
        assert!(hidden(&key, Side::Player, 0));
        let quest = Item {
            quest_item: true,
            ..gun.clone()
        };
        assert!(hidden(&quest, Side::Player, 0));
        assert!(!hidden(&quest, Side::Container, 0));
        let regen = Item {
            regenerating_ammo: true,
            ..ammo.clone()
        };
        assert!(hidden(&regen, Side::Container, 0));
        // Not listed at all: no name, not playable.
        assert!(!listed(&item(7, "", 1, WEAP)));
        assert!(!listed(&Item {
            playable: false,
            ..gun
        }));
    }

    /// `0075c280`, `0075d160`: lines "name (count)", the worn mark, sorted
    /// by their text byte by byte; the weight line rounded.
    #[test]
    fn filling_labelling_and_sorting() {
        let mut worn = item(3, "Leather Armor", 1, form_type::ARMO);
        worn.equipped = true;
        let player = vec![
            item(1, "Stimpak", 4, form_type::ALCH),
            item(2, "bottle cap", 1, 0x1F),
            worn,
            item(4, "Key", 1, form_type::KEYM),
        ];
        let (mut ui, m) = opened(player, vec![item(9, "Tin Can", 2, 0x1F)], "Box");
        assert_eq!(
            texts(&mut ui, &m, Side::Player),
            ["Leather Armor", "Stimpak (4)", "bottle cap"]
        );
        assert_eq!(texts(&mut ui, &m, Side::Container), ["Tin Can (2)"]);
        let line = m.lists[0].shown_items(&mut ui, 0, i32::MAX)[0];
        let mark = ui
            .find_below(line.tile, "CM_list_template_ItemMarker")
            .unwrap();
        assert_eq!(ui.number(mark, t::VISIBLE), 1.0);
        let text = ui.find_below(line.tile, "ListItemText").unwrap();
        assert_eq!(ui.string(text, t::STRING).as_deref(), Some("Leather Armor"));
        let weight = m.tiles[3].unwrap();
        assert_eq!(ui.string(weight, t::STRING).as_deref(), Some("Wg   13/199"));
        assert_eq!(
            ui.string(m.tiles[6].unwrap(), t::STRING).as_deref(),
            Some("Box")
        );
        assert_eq!(
            ui.string(m.tiles[10].unwrap(), t::STRING).as_deref(),
            Some("Take All")
        );
    }

    /// `0075e8c0`: a name too wide for the title is cut with "...".
    #[test]
    fn a_long_name_is_cut() {
        // The frame is 464 wide: 464 - 80 - 35 = 349 for the name; the test
        // font's letters are 10 wide, dots 10.
        let long = "A".repeat(40);
        let (mut ui, m) = opened(vec![], vec![item(1, "Can", 1, 0x1F)], &long);
        let s = ui.string(m.tiles[6].unwrap(), t::STRING).unwrap();
        // 30 for the dots, then letters while the width is at most 349:
        // 32 letters (the last taking it to 350).
        assert_eq!(s, format!("{}...", "A".repeat(32)));
    }

    /// `0075be80`: picking an item moves one, more than five asks how
    /// many, weightless things from the container go whole; Take All moves
    /// what the container's list shows and closes.
    #[test]
    fn clicks_move_items() {
        let mut caps = item(15, "Bottle Cap", 130, 0x1F);
        caps.weightless = true;
        let container = vec![caps, item(2, "Can", 1, 0x1F), item(3, "Rounds", 40, 0x29)];
        let (mut ui, mut m) = opened(vec![item(7, "Rock", 9, 0x1F)], container, "Box");
        let lines = m.lists[1].shown_items(&mut ui, 0, i32::MAX);
        let mut i = crate::menu::Interface::default();
        let menu = m.menu;
        // The pointer onto the caps' line (first: "Bottle Cap (130)").
        i.choose(&mut ui, &mut m, Some(lines[0].tile), false);
        assert_eq!(m.current, Some(Side::Container));
        assert_eq!(m.selected, Some((Side::Container, 0)));
        assert_eq!(ui.number(m.tiles[9].unwrap(), t::USER0), 2.0);
        assert_eq!(ui.number(m.tiles[12].unwrap(), t::VISIBLE), 1.0);
        i.click(&mut ui, menu, &mut m, lines[0].tile, 0.0);
        assert_eq!(
            m.requests.pop(),
            Some(Request::Move {
                from: Side::Container,
                form: 15,
                count: 130
            })
        );
        // Forty rounds: how many?
        i.choose(&mut ui, &mut m, Some(lines[2].tile), false);
        i.click(&mut ui, menu, &mut m, lines[2].tile, 0.0);
        assert_eq!(
            m.requests.pop(),
            Some(Request::AskQuantity {
                from: Side::Container,
                form: 3,
                most: 40
            })
        );
        m.quantity_chosen(&mut ui, 12);
        assert_eq!(
            m.requests.pop(),
            Some(Request::Move {
                from: Side::Container,
                form: 3,
                count: 12
            })
        );
        // The player's rocks: nine is more than five.
        let rock = m.lists[0].shown_items(&mut ui, 0, i32::MAX)[0];
        i.choose(&mut ui, &mut m, Some(rock.tile), false);
        assert_eq!(m.current, Some(Side::Player));
        assert_eq!(ui.number(m.tiles[9].unwrap(), t::USER0), 1.0);
        i.click(&mut ui, menu, &mut m, rock.tile, 0.0);
        assert!(matches!(
            m.requests.pop(),
            Some(Request::AskQuantity { most: 9, .. })
        ));
        // Take All: every shown line, whole, then the menu closes.
        m.click(&mut ui, 10, None, 0.0);
        assert_eq!(m.requests.len(), 4);
        assert_eq!(m.requests[3], Request::Close);
        assert!(m.sounds.contains(&TAKE_ALL_SOUND.to_string()));
        assert!(m.closed);
    }

    /// The filter arrows and the title's wheel step on past empty filters
    /// (`0075be80`, `0075d4a0`); titles in upper case.
    #[test]
    fn stepping_filters() {
        let container = vec![
            item(1, "Pistol", 1, form_type::WEAP),
            item(2, "Can", 1, 0x1F),
        ];
        let (mut ui, mut m) = opened(vec![], container, "Box");
        m.click(&mut ui, 7, None, 0.0);
        assert_eq!(m.filters[1], 1);
        assert_eq!(texts(&mut ui, &m, Side::Container), ["Pistol"]);
        assert_eq!(
            ui.string(m.tiles[6].unwrap(), t::STRING).as_deref(),
            Some("WEAPONS")
        );
        // On: apparel and aid are empty, misc has the can.
        m.click(&mut ui, 7, None, 0.0);
        assert_eq!(m.filters[1], 4);
        assert_eq!(texts(&mut ui, &m, Side::Container), ["Can"]);
        // The wheel down over the title steps back.
        m.wheel(&mut ui, 6, m.tiles[6].unwrap(), -120);
        assert_eq!(m.filters[1], 1);
        // The player's side has nothing: every filter is skipped round to 0.
        m.click(&mut ui, 2, None, 0.0);
        assert_eq!(m.filters[0], 0);
        assert_eq!(
            ui.string(m.tiles[1].unwrap(), t::STRING).as_deref(),
            Some("ITEMS")
        );
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuPrevNext"));
    }

    /// After a move the changed item's lines are made again; a current
    /// side left empty gives way to the other (`0075c280`).
    #[test]
    fn refreshing_after_a_move() {
        let (mut ui, mut m) = opened(vec![], vec![item(2, "Can", 1, 0x1F)], "Box");
        let line = m.lists[1].shown_items(&mut ui, 0, i32::MAX)[0];
        let mut i = crate::menu::Interface::default();
        i.choose(&mut ui, &mut m, Some(line.tile), false);
        assert_eq!(m.current, Some(Side::Container));
        m.refresh_item(&mut ui, 2, vec![item(2, "Can", 1, 0x1F)], vec![]);
        assert_eq!(texts(&mut ui, &m, Side::Player), ["Can"]);
        assert!(texts(&mut ui, &m, Side::Container).is_empty());
        assert_eq!(m.current, Some(Side::Player));
        assert_eq!(m.selected, None);
        // The space bar closes it.
        assert!(m.key(&mut ui, 0x20, 0.0));
        assert!(m.closed);
    }

    /// `0075bc80` mode 3 (a companion's things): Take All is hidden.
    #[test]
    fn a_companions_things_have_no_take_all() {
        let mut ui = test_support::ui();
        let mut m = ContainerMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::container_menu(), &mut m);
        m.name = "Cass".into();
        m.mode = 3;
        assert!(m.open(&mut ui, Vec::new(), Vec::new()));
        assert_eq!(ui.number(m.tiles[10].unwrap(), t::VISIBLE), 0.0);
        let (mut ui, m) = opened(Vec::new(), Vec::new(), "Box");
        assert_ne!(ui.number(m.tiles[10].unwrap(), t::VISIBLE), 0.0);
    }

    /// `CM_Subtitle` (`0075b310`, `0075eea0`, `0075eac0`): emptied on
    /// opening, a line on it until its time is up.
    #[test]
    fn a_companions_line_on_the_subtitle() {
        let (mut ui, mut m) = opened(Vec::new(), Vec::new(), "Cass");
        let sub = ui.find(m.menu, "CM_Subtitle").unwrap();
        assert_eq!(ui.string(sub, t::STRING).unwrap_or_default(), "");
        m.say(&mut ui, "What do you need?", 2000.0);
        assert_eq!(ui.string(sub, t::STRING).unwrap(), "What do you need?");
        m.update(&mut ui, 1500.0);
        assert_eq!(ui.string(sub, t::STRING).unwrap(), "What do you need?");
        m.update(&mut ui, 2001.0);
        assert_eq!(ui.string(sub, t::STRING).unwrap_or_default(), "");
        assert_eq!(m.subtitle_until, None);
    }
}

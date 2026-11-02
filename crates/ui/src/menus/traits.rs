//! Traits (`menus\trait_menu.xml`, class `TraitMenu` 1084, vtable
//! `010779bc` in FalloutNV.exe): the list of traits a new character may
//! take, up to `iTraitMenuMaxNumTraits`. Read from the code (`007e6990` and
//! the functions after it), a sibling of the level-up menu's perk page:
//!
//! * opening (`007e6990`): fails without tile ids 0–8 (`007e6dd0`); the
//!   most traits from `iTraitMenuMaxNumTraits` (`011d2f94`); the title (id
//!   0) `sTraitMenuTitleTextSingular` ("CHOOSE %d TRAIT") with 1 when the
//!   most is 1, else `sTraitMenuTitleText` ("CHOOSE UP TO %d TRAITS") with
//!   the most; Reset (5) `sReset`, Done (6) `_Title` `sDone`; the traits
//!   (`world::perks::trait_choices`; with `bHideUnavailablePerks` only
//!   those that can be taken) as the level-up menu's perk lines with id 10
//!   (`007e7030`); `_CurrPoints` 0, `_MaxPoints` the most; the counter; each
//!   line's `_TextAlpha` and the sort as the perk page's.
//! * the counter (`007e71b0`, id 4, always shown): what's left;
//!   `sTraitMenuCounterSingular` ("%d TRAIT LEFT") for 1, else
//!   `sTraitMenuCounter` ("%d TRAITS LEFT"), the number put in.
//! * clicks (`007e7280`): a trait (10) picked again is let go; a bright one
//!   is picked while fewer than the most are; Reset (5) sound 1 and lets
//!   them all go; Done (6) closes and the traits picked are given
//!   (`007e6e10`, `007e76e0`: each one's next rank).
//! * the pointer on a trait (`007e6ea0`): its picture in
//!   `LUM_SelectionIcon` (shown when its path has 2 or more letters), its
//!   requirements and description (`005ebac0`) in `TM_DescriptionText`,
//!   the badge hidden, sound 4, the description's scroll bar back to the
//!   top (`_current_value` 0, `_SetInCode`).
//!
//! Not here: the controller's buttons (special 9, the right stick
//! scrolling the description, `007e73f0`; 0xb, `007e75d0`).

use crate::list::ListBox;
use crate::menu::{self, MenuCode};
use crate::menus::levelup::{self, Perk};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\trait_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1084;
/// A trait line's id.
pub const TRAIT_ID: i32 = 10;

/// What the menu asks of the game.
#[derive(Debug, Clone, PartialEq)]
pub enum Request {
    /// The menu closed: the traits picked (forms), each given its next rank.
    Close { traits: Vec<u32> },
}

/// The trait menu.
#[derive(Debug)]
pub struct TraitMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..8 (`+0x28`).
    pub tiles: [Option<TileId>; 9],
    /// The list (`+0x54`).
    pub list: ListBox,
    pub traits: Vec<Perk>,
    /// Picked (`+0x4c`) and the most (`+0x50`).
    pub points: i32,
    pub most: i32,
    pub requests: Vec<Request>,
    pub sounds: Vec<String>,
    pub closed: bool,
}

fn custom(ui: &mut Ui, name: &str) -> i32 {
    ui.names.lookup_or_add(name).unwrap_or(0)
}

impl TraitMenu {
    pub fn new(menu: TileId) -> TraitMenu {
        TraitMenu {
            menu,
            tiles: [None; 9],
            list: ListBox::default(),
            traits: Vec::new(),
            points: 0,
            most: 0,
            requests: Vec::new(),
            sounds: Vec::new(),
            closed: false,
        }
    }

    fn tile(&self, id: usize) -> Option<TileId> {
        self.tiles.get(id).copied().flatten()
    }

    /// Opens it (`007e6990`) with `most` traits allowed; false without its
    /// tiles.
    pub fn open(&mut self, ui: &mut Ui, most: i32, traits: Vec<Perk>) -> bool {
        if self.tiles.iter().any(Option::is_none) {
            return false;
        }
        self.points = 0;
        self.most = most;
        let text = |ui: &Ui, n: &str| ui.setting_text(n).unwrap_or_default();
        if let Some(title) = self.tile(0) {
            let s = if most == 1 {
                text(ui, "sTraitMenuTitleTextSingular").replacen("%d", "1", 1)
            } else {
                text(ui, "sTraitMenuTitleText").replacen("%d", &most.to_string(), 1)
            };
            ui.set_text(title, t::STRING, &s);
        }
        if let Some(reset) = self.tile(5) {
            let s = text(ui, "sReset");
            ui.set_text(reset, t::STRING, &s);
        }
        if let Some(done) = self.tile(6) {
            let title = custom(ui, "_Title");
            let s = text(ui, "sDone");
            ui.set_string(done, title, &s);
        }
        self.traits = traits;
        if let Some(list) = self.tile(1) {
            self.list = ListBox::new(ui, list, "LUM_PerkTemplate");
        }
        levelup::add_perk_lines(ui, self.menu, &mut self.list, &self.traits, TRAIT_ID);
        self.set_points(ui);
        let most_id = custom(ui, "_MaxPoints");
        ui.set_number(self.menu, most_id, most as f32);
        self.counter(ui);
        levelup::brighten_and_sort(ui, &mut self.list, &self.traits);
        self.list.select(ui, None);
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
        true
    }

    fn set_points(&mut self, ui: &mut Ui) {
        let curr = custom(ui, "_CurrPoints");
        ui.set_number(self.menu, curr, self.points as f32);
    }

    /// The counter (`007e71b0`).
    fn counter(&mut self, ui: &mut Ui) {
        let Some(counter) = self.tile(4) else {
            return;
        };
        let left = self.most - self.points;
        let name = if left == 1 {
            "sTraitMenuCounterSingular"
        } else {
            "sTraitMenuCounter"
        };
        let f = ui.setting_text(name).unwrap_or_default();
        ui.set_text(counter, t::STRING, &f.replacen("%d", &left.to_string(), 1));
        ui.set_number(counter, t::VISIBLE, 1.0);
        ui.refresh();
    }

    /// The traits picked, their forms.
    pub fn picked(&self, ui: &mut Ui) -> Vec<u32> {
        levelup::picked(ui, &self.list)
            .into_iter()
            .filter_map(|tile| self.list.value_of(tile))
            .filter_map(|v| self.traits.get(v as usize))
            .map(|p| p.form)
            .collect()
    }

    /// Done (`007e6e10`).
    fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        let traits = self.picked(ui);
        self.requests.push(Request::Close { traits });
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }
}

impl MenuCode for TraitMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..9).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `007e7280`.
    fn click(&mut self, ui: &mut Ui, id: i32, tile: Option<TileId>, _now: f64) {
        let sel = custom(ui, "_selected");
        match id {
            5 => {
                if let Some(s) = menu::menu_sound(1) {
                    self.sounds.push(s.to_string());
                }
                self.points = 0;
                self.set_points(ui);
                for line in levelup::picked(ui, &self.list) {
                    ui.set_number(line, sel, 0.0);
                }
                self.counter(ui);
            }
            6 => self.close(ui),
            TRAIT_ID => {
                let Some(line) = tile else {
                    return;
                };
                let alpha = custom(ui, "_TextAlpha");
                if levelup::picked(ui, &self.list).contains(&line) {
                    ui.set_number(line, sel, 0.0);
                } else if ui.number(line, alpha) == 255.0 && self.points < self.most {
                    ui.set_number(line, sel, 1.0);
                }
                self.points = levelup::picked(ui, &self.list).len() as i32;
                self.set_points(ui);
                self.counter(ui);
            }
            _ => {}
        }
        ui.refresh();
    }

    /// `007e6ea0`.
    fn mouseover(&mut self, ui: &mut Ui, id: i32, tile: TileId) {
        if id != TRAIT_ID {
            return;
        }
        let Some(p) = self
            .list
            .value_of(tile)
            .and_then(|v| self.traits.get(v as usize))
            .cloned()
        else {
            return;
        };
        if let Some(icon) = self.tile(2) {
            ui.set_string(icon, t::FILENAME, &p.icon);
            let shown = p.icon.len() >= 2;
            ui.set_number(icon, t::VISIBLE, if shown { 1.0 } else { 0.0 });
        }
        if let Some(text) = self.tile(3) {
            ui.set_text(text, t::STRING, &p.text);
            ui.set_number(text, t::VISIBLE, 1.0);
        }
        if let Some(badge) = self.tile(7) {
            ui.set_number(badge, t::VISIBLE, 0.0);
        }
        if let Some(s) = menu::menu_sound(4) {
            self.sounds.push(s.to_string());
        }
        if let Some(bar) = self.tile(8) {
            let (value, set) = (custom(ui, "_current_value"), custom(ui, "_SetInCode"));
            crate::list::set_keeping_operators(ui, bar, value, 0.0);
            ui.set_number(bar, set, 1.0);
        }
        ui.refresh();
    }

    fn lists(&mut self) -> Vec<&mut ListBox> {
        vec![&mut self.list]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn perk(form: u32, name: &str, available: bool) -> Perk {
        Perk {
            form,
            name: name.into(),
            rank: 1,
            ranks: 1,
            min_level: 1,
            available,
            icon: format!("Interface\\Icons\\{name}.dds"),
            text: format!("{name} does things."),
        }
    }

    fn opened(most: i32, traits: Vec<Perk>) -> (Ui, TraitMenu) {
        let mut ui = test_support::ui();
        let mut m = TraitMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::trait_menu(), &mut m);
        assert!(m.open(&mut ui, most, traits));
        (ui, m)
    }

    fn line(ui: &mut Ui, m: &TraitMenu, text: &str) -> TileId {
        m.list
            .shown_items(ui, 0, i32::MAX)
            .into_iter()
            .find(|l| ui.string(l.tile, t::STRING).as_deref() == Some(text))
            .unwrap()
            .tile
    }

    /// `007e6990`, `007e71b0`, `007e7280`: the title and counter by the
    /// number allowed, picking up to the most, Reset, Done.
    #[test]
    fn picking_traits() {
        let traits = vec![
            perk(0x801, "Wild Wasteland", true),
            perk(0x802, "Built to Destroy", true),
            perk(0x803, "Kamikaze", true),
            perk(0x804, "Locked Away", false),
        ];
        let (mut ui, mut m) = opened(2, traits);
        let (title, counter) = (m.tiles[0].unwrap(), m.tiles[4].unwrap());
        assert_eq!(
            ui.string(title, t::STRING).as_deref(),
            Some("CHOOSE UP TO 2 TRAITS")
        );
        assert_eq!(
            ui.string(counter, t::STRING).as_deref(),
            Some("2 TRAITS LEFT")
        );
        let names: Vec<String> = m
            .list
            .shown_items(&mut ui, 0, i32::MAX)
            .iter()
            .map(|l| ui.string(l.tile, t::STRING).unwrap_or_default())
            .collect();
        assert_eq!(
            names,
            [
                "Built to Destroy",
                "Kamikaze",
                "Wild Wasteland",
                "Locked Away"
            ]
        );
        for name in [
            "Kamikaze",
            "Locked Away",
            "Wild Wasteland",
            "Built to Destroy",
        ] {
            let l = line(&mut ui, &m, name);
            m.click(&mut ui, TRAIT_ID, Some(l), 0.0);
        }
        // The dim one can't be taken, and the third bright one is one too
        // many.
        assert_eq!(m.points, 2);
        assert_eq!(
            ui.string(counter, t::STRING).as_deref(),
            Some("0 TRAITS LEFT")
        );
        let kamikaze = line(&mut ui, &m, "Kamikaze");
        m.click(&mut ui, TRAIT_ID, Some(kamikaze), 0.0);
        assert_eq!(
            ui.string(counter, t::STRING).as_deref(),
            Some("1 TRAIT LEFT")
        );
        m.mouseover(&mut ui, TRAIT_ID, kamikaze);
        assert_eq!(
            ui.string(m.tiles[3].unwrap(), t::STRING).as_deref(),
            Some("Kamikaze does things.")
        );
        assert_eq!(ui.number(m.tiles[2].unwrap(), t::VISIBLE), 1.0);
        m.click(&mut ui, 5, None, 0.0);
        assert_eq!(m.points, 0);
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuOK"));
        let wild = line(&mut ui, &m, "Wild Wasteland");
        m.click(&mut ui, TRAIT_ID, Some(wild), 0.0);
        m.click(&mut ui, 6, None, 0.0);
        assert!(m.closed);
        assert_eq!(
            m.requests,
            [Request::Close {
                traits: vec![0x801]
            }]
        );
        // One allowed: the singular title.
        let (ui, m) = opened(1, vec![perk(0x801, "Wild Wasteland", true)]);
        let mut ui = ui;
        assert_eq!(
            ui.string(m.tiles[0].unwrap(), t::STRING).as_deref(),
            Some("CHOOSE 1 TRAIT")
        );
        assert_eq!(
            ui.string(m.tiles[4].unwrap(), t::STRING).as_deref(),
            Some("1 TRAIT LEFT")
        );
    }
}

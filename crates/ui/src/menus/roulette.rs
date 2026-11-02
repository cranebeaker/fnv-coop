//! Roulette's tiles (`menus\roulette_menu.xml`, class `RouletteMenu` 1082 in
//! FalloutNV.exe). The game runs in `world::casino::roulette::Roulette`;
//! this shows it as the menu's code does (`Create` `007bbe20` writes the
//! texts, `SetTileTextAlpha` `007bf710` the buttons' dimming, `DoIdle`
//! `007bd2a0` and `DoClick` `007bc870` the numbers and the status line).
//! Tile ids (`AttachTileByID` `007b8f40`, 0 to 10):
//!
//! * 0 the bet: `_Value` `sCurrentBetText`, `_X` the current bet (the idle
//!   clamp writes the bet into `_Value` instead);
//! * 10 the table's total: `_Value` `sTotalBetText`, `_X` the total;
//! * 1 the chips: `_Value` `sChipCountText`, `_X` the menu's chips;
//! * 2 the casino: `_Value` the earnings line, its child
//!   `ROM_TotalEarningsValue` hidden;
//! * 3 to 8 `sPlaceBetText`, `sRemoveBetText`, `sFinsihBetText`,
//!   `sIncreaseBetText`, `sDecreaseBetText`, `sExit` (W, F, S, E, Q, R); a
//!   button that can't be used now has its children's `alpha` at 128;
//! * 9 the status line: the spot under the cursor, a round's result.

use std::collections::HashMap;

use crate::menu::MenuCode;
use crate::names::t;
use crate::tile::{TileId, Ui};
use world::casino::roulette::{tile as id, Roulette};

/// The menu's file.
pub const FILE: &str = "menus\\roulette_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1082;
/// How many tile ids it keeps.
pub const TILE_COUNT: usize = 11;

/// The buttons' labels (settings and their exe defaults).
const LABELS: [(i32, &str, &str); 6] = [
    (id::PLACE_BET, "sPlaceBetText", "Place Bet"),
    (id::REMOVE_BET, "sRemoveBetText", "Remove Bet"),
    (id::FINISH_BET, "sFinsihBetText", "Finish Bet"),
    (id::INCREASE_BET, "sIncreaseBetText", "Increase Bet"),
    (id::DECREASE_BET, "sDecreaseBetText", "Decrease Bet"),
    (id::EXIT, "sExit", "Exit"),
];

/// The menu's tiles and what was done to them.
#[derive(Debug)]
pub struct RouletteMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; TILE_COUNT],
    /// Clicks for the game, oldest first.
    pub clicks: Vec<i32>,
    numbers: HashMap<i32, i64>,
    texts: HashMap<i32, String>,
    alphas: [f32; TILE_COUNT],
}

impl RouletteMenu {
    pub fn new(menu: TileId) -> RouletteMenu {
        RouletteMenu {
            menu,
            tiles: [None; TILE_COUNT],
            clicks: Vec::new(),
            numbers: HashMap::new(),
            texts: HashMap::new(),
            alphas: [-1.0; TILE_COUNT],
        }
    }

    pub fn tile(&self, id: i32) -> Option<TileId> {
        usize::try_from(id)
            .ok()
            .and_then(|i| self.tiles.get(i).copied().flatten())
    }

    fn setting(ui: &Ui, name: &str, default: &str) -> String {
        ui.setting_text(name).unwrap_or_else(|| default.to_string())
    }

    /// `_X`: 1, worked out, then the number.
    fn set_x(&mut self, ui: &mut Ui, id: i32, v: i64) {
        if self.numbers.get(&id) == Some(&v) {
            return;
        }
        if let Some(tile) = self.tile(id) {
            self.numbers.insert(id, v);
            let x = ui.names.lookup_or_add("_X").unwrap_or(0);
            ui.set_number(tile, x, 1.0);
            ui.work_out_all(tile);
            ui.set_number(tile, x, v as f32);
            ui.work_out_all(tile);
        }
    }

    fn set_value_text(&mut self, ui: &mut Ui, id: i32, s: &str) {
        if self.texts.get(&id).is_some_and(|old| old == s) {
            return;
        }
        if let Some(tile) = self.tile(id) {
            self.texts.insert(id, s.to_string());
            let value = ui.names.lookup_or_add("_Value").unwrap_or(0);
            ui.set_number(tile, value, 1.0);
            ui.set_text(tile, value, s);
        }
    }

    /// `SetTileTextAlpha` (`007bf710`): every child's alpha.
    fn set_alpha(&mut self, ui: &mut Ui, id: i32, a: f32) {
        let Some(slot) = usize::try_from(id).ok().filter(|&i| i < TILE_COUNT) else {
            return;
        };
        if self.alphas[slot] == a {
            return;
        }
        self.alphas[slot] = a;
        if let Some(tile) = self.tile(id) {
            for c in ui.tiles[tile].children.clone() {
                ui.set_number(c, t::ALPHA, a);
            }
        }
    }

    /// Shows the menu with `Create`'s texts.
    pub fn open(&mut self, ui: &mut Ui) {
        for (tile, name, default) in [
            (id::CURRENT_BET, "sCurrentBetText", "Current Bet: "),
            (id::TOTAL_BET, "sTotalBetText", "Total Bet: "),
            (id::CHIP_COUNT, "sChipCountText", "Chips: "),
        ] {
            let s = Self::setting(ui, name, default);
            self.set_value_text(ui, tile, &s);
        }
        if let Some(info) = self.tile(id::CASINO_INFO) {
            if let Some(v) = ui.find_below(info, "ROM_TotalEarningsValue") {
                ui.set_number(v, t::VISIBLE, 0.0);
            }
        }
        for (tile, name, default) in LABELS {
            let s = Self::setting(ui, name, default);
            if let Some(tile) = self.tile(tile) {
                ui.set_number(tile, t::STRING, 1.0);
                ui.set_text(tile, t::STRING, &s);
            }
        }
        if let Some(tile) = self.tile(id::STATUS) {
            ui.set_text(tile, t::STRING, " ");
            ui.set_number(tile, t::VISIBLE, 1.0);
        }
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
    }

    /// The tiles for the game as it is now; `status` the status line's
    /// words, worked out by the caller.
    pub fn fill(&mut self, ui: &mut Ui, m: &Roulette, earnings: &str, status: Option<&str>) {
        match m.bet_in_label {
            Some(v) => self.set_value_text(ui, id::CURRENT_BET, &v.to_string()),
            None => self.set_x(ui, id::CURRENT_BET, i64::from(m.current_bet)),
        }
        self.set_x(ui, id::TOTAL_BET, i64::from(m.total_bet));
        self.set_x(ui, id::CHIP_COUNT, i64::from(m.chips));
        self.set_value_text(ui, id::CASINO_INFO, earnings);
        for tile in id::PLACE_BET..=id::EXIT {
            self.set_alpha(ui, tile, m.alpha[tile as usize]);
        }
        if let Some(tile) = self.tile(id::STATUS) {
            if let Some(s) = status {
                if self.texts.get(&id::STATUS).map_or(true, |old| old != s) {
                    self.texts.insert(id::STATUS, s.to_string());
                    ui.set_text(tile, t::STRING, s);
                }
            }
            ui.set_number(tile, t::VISIBLE, if m.status_shown { 1.0 } else { 0.0 });
        }
        ui.refresh();
    }
}

impl MenuCode for RouletteMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if let Some(slot) = usize::try_from(id).ok().and_then(|i| self.tiles.get_mut(i)) {
            *slot = Some(tile);
        }
    }

    fn click(&mut self, _ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        self.clicks.push(id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;
    use world::casino::roulette::{spots, ALPHA_DIM};
    use world::casino::CasinoData;

    #[test]
    fn texts_dimming_and_keys() {
        let (mut ui, mut menu) = test_support::roulette_menu();
        menu.open(&mut ui);
        let text = |_: &str, d: &str| d.to_string();
        let (m, _) = Roulette::open(
            9000,
            1,
            100,
            42,
            5.0,
            CasinoData::default(),
            spots(&text),
            Vec::new(),
        );
        menu.fill(&mut ui, &m, "Gommorah Earnings: 0", None);
        let place = menu.tile(id::PLACE_BET).unwrap();
        assert_eq!(ui.string(place, t::STRING).as_deref(), Some("Place Bet"));
        let finish = menu.tile(id::FINISH_BET).unwrap();
        let child = ui.tiles[finish].children[0];
        assert_eq!(ui.number(child, t::ALPHA), ALPHA_DIM);
        let x = ui.names.lookup("_X").unwrap();
        let total = menu.tile(id::TOTAL_BET).unwrap();
        assert_eq!(ui.number(total, x), 0.0);
        let chips = menu.tile(id::CHIP_COUNT).unwrap();
        assert_eq!(ui.number(chips, x), 42.0);
        // W places, S spins, F removes (the menu's `_PCButton_` traits).
        let mut interface = crate::menu::Interface::default();
        for key in *b"wsf" {
            interface.key(
                &mut ui,
                menu.menu,
                &mut menu,
                u32::from(key),
                false,
                false,
                0.0,
            );
        }
        assert_eq!(menu.clicks, [id::PLACE_BET, id::FINISH_BET, id::REMOVE_BET]);
    }
}

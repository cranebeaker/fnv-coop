//! Blackjack's tiles (`menus\black_jack_menu.xml`, class `BlackJackMenu`
//! 1081 in FalloutNV.exe). The game runs in `world::casino::blackjack::
//! Blackjack`; this shows it as the menu's code does (`Create` `00733630`
//! writes the texts, `SetBetTiles` `00738cc0` the buttons' labels,
//! `SetTileTextAlpha` `00738c30` their dimming, `DoIdle` `00734db0` and
//! `DoClick` `00733ff0` the numbers and the status line). Tile ids
//! (`AttachTileByID` `007317a0`, 0 to 9):
//!
//! * 0 the bet: `_Value` `sCurrentBetText`, `_x` the bet (both hands' while
//!   split or doubled);
//! * 1 the chips: `_Value` `sChipCountText`, `_x` the menu's chips;
//! * 2 the casino: `_Value` the earnings line, its child
//!   `BJM_TotalEarningsValue` hidden;
//! * 3 Hit (`sHitText`, shown only while playing);
//! * 4 to 8 betting `sDealText`, `sIncreaseBetText`, `sDecreaseBetText`,
//!   `sBetMax`, `sExit`; playing `sDoubleDownText`, `sSplitText`,
//!   `sSwitchHandsText`, `sSurrenderText`, `sStayText` (W, E, Q, S, R; Hit
//!   F). A button that can't be used now has its direct children's `alpha`
//!   at 128 (still clickable);
//! * 9 the status line: a round's result, "You've reached the max hand
//!   size".

use std::collections::HashMap;

use crate::menu::MenuCode;
use crate::names::t;
use crate::tile::{TileId, Ui};
use world::casino::blackjack::{tile as id, Blackjack};

/// The menu's file.
pub const FILE: &str = "menus\\black_jack_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1081;
/// How many tile ids it keeps.
pub const TILE_COUNT: usize = 10;
/// "You've reached the max hand size" (`01070cc0`, not a setting).
pub const MAX_HAND_TEXT: &str = "You've reached the max hand size";

/// The labels of buttons 4 to 8: betting, then playing (settings and their
/// exe defaults).
const LABELS: [[(&str, &str); 5]; 2] = [
    [
        ("sDealText", "Deal"),
        ("sIncreaseBetText", "Increase Bet"),
        ("sDecreaseBetText", "Decrease Bet"),
        ("sBetMax", "Bet Max"),
        ("sExit", "Exit"),
    ],
    [
        ("sDoubleDownText", "Double Down"),
        ("sSplitText", "Split"),
        ("sSwitchHandsText", "Switch Hands"),
        ("sSurrenderText", "Surrender"),
        ("sStayText", "Stay"),
    ],
];

/// The menu's tiles and what was done to them.
#[derive(Debug)]
pub struct BlackjackMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; TILE_COUNT],
    /// Clicks for the game, oldest first.
    pub clicks: Vec<i32>,
    numbers: HashMap<i32, i64>,
    texts: HashMap<i32, String>,
    labels: Option<bool>,
    alphas: [f32; TILE_COUNT],
}

impl BlackjackMenu {
    pub fn new(menu: TileId) -> BlackjackMenu {
        BlackjackMenu {
            menu,
            tiles: [None; TILE_COUNT],
            clicks: Vec::new(),
            numbers: HashMap::new(),
            texts: HashMap::new(),
            labels: None,
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

    /// `_x`: 1, worked out, then the number.
    fn set_x(&mut self, ui: &mut Ui, id: i32, v: i64) {
        if self.numbers.get(&id) == Some(&v) {
            return;
        }
        if let Some(tile) = self.tile(id) {
            self.numbers.insert(id, v);
            let x = ui.names.lookup_or_add("_x").unwrap_or(0);
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

    /// A tile's `string` (after a 1, `00a01290`).
    fn set_string(&mut self, ui: &mut Ui, id: i32, s: &str) {
        if let Some(tile) = self.tile(id) {
            ui.set_number(tile, t::STRING, 1.0);
            ui.set_text(tile, t::STRING, s);
        }
    }

    /// `SetTileTextAlpha` (`00738c30`): every direct child's alpha.
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
        let bet = Self::setting(ui, "sCurrentBetText", "Current Bet: ");
        self.set_value_text(ui, id::CURRENT_BET, &bet);
        let chips = Self::setting(ui, "sChipCountText", "Chips: ");
        self.set_value_text(ui, id::CHIP_COUNT, &chips);
        if let Some(info) = self.tile(id::CASINO_INFO) {
            if let Some(v) = ui.find_below(info, "BJM_TotalEarningsValue") {
                ui.set_number(v, t::VISIBLE, 0.0);
            }
        }
        let hit = Self::setting(ui, "sHitText", "Hit");
        if let Some(tile) = self.tile(id::HIT) {
            ui.set_text(tile, t::STRING, &hit);
        }
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
    }

    /// The tiles for the game as it is now: the numbers, the labels and
    /// their dimming, the earnings line, the status line (`status`: its
    /// words, worked out by the caller).
    pub fn fill(&mut self, ui: &mut Ui, m: &Blackjack, earnings: &str, status: Option<&str>) {
        self.set_x(ui, id::CURRENT_BET, i64::from(m.bet_shown));
        self.set_x(ui, id::CHIP_COUNT, i64::from(m.chips));
        self.set_value_text(ui, id::CASINO_INFO, earnings);
        if self.labels != Some(m.betting_labels) {
            self.labels = Some(m.betting_labels);
            let set = &LABELS[usize::from(!m.betting_labels)];
            for (i, (name, default)) in set.iter().enumerate() {
                let s = Self::setting(ui, name, default);
                self.set_string(ui, id::DOUBLE_DEAL + i as i32, &s);
            }
            if let Some(tile) = self.tile(id::HIT) {
                let shown = if m.betting_labels { 0.0 } else { 1.0 };
                ui.set_number(tile, t::VISIBLE, shown);
            }
        }
        for tile in id::HIT..=id::STAY_EXIT {
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

impl MenuCode for BlackjackMenu {
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
    use world::casino::blackjack::{ALPHA_DIM, ALPHA_FULL};
    use world::casino::CasinoData;

    #[test]
    fn labels_dimming_and_keys() {
        let (mut ui, mut menu) = test_support::blackjack_menu();
        menu.open(&mut ui);
        let (mut m, _) = Blackjack::open(
            1.5,
            0.2,
            3,
            false,
            9000,
            1,
            200,
            42,
            5.0,
            CasinoData::default(),
        );
        menu.fill(&mut ui, &m, "Gommorah Earnings: 0", None);
        let deal = menu.tile(id::DOUBLE_DEAL).unwrap();
        assert_eq!(ui.string(deal, t::STRING).as_deref(), Some("Deal"));
        let hit = menu.tile(id::HIT).unwrap();
        assert_eq!(ui.number(hit, t::VISIBLE), 0.0);
        let x = ui.names.lookup("_x").unwrap();
        let chips = menu.tile(id::CHIP_COUNT).unwrap();
        assert_eq!(ui.number(chips, x), 42.0);
        // Playing: the other labels, Hit shown, Split dimmed.
        m.betting_labels = false;
        m.alpha[id::SPLIT_INCREASE_BET as usize] = ALPHA_DIM;
        menu.fill(&mut ui, &m, "Gommorah Earnings: 0", None);
        assert_eq!(ui.string(deal, t::STRING).as_deref(), Some("Double Down"));
        assert_eq!(ui.number(hit, t::VISIBLE), 1.0);
        let split = menu.tile(id::SPLIT_INCREASE_BET).unwrap();
        let child = ui.tiles[split].children[0];
        assert_eq!(ui.number(child, t::ALPHA), ALPHA_DIM);
        let stay = menu.tile(id::STAY_EXIT).unwrap();
        let child = ui.tiles[stay].children[0];
        assert_eq!(ui.number(child, t::ALPHA), ALPHA_FULL);
        // F hits (the menu's `_PCButton_F`), R stays (`_PCBUTTON_R`).
        let mut interface = crate::menu::Interface::default();
        for key in *b"fr" {
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
        assert_eq!(menu.clicks, [id::HIT, id::STAY_EXIT]);
    }
}

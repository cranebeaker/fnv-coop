//! The slot machine's tiles (`menus\slot_machine_menu.xml`, class
//! `SlotMachineMenu` 1080 in FalloutNV.exe). The machine runs in
//! `world::casino::slots::SlotMachine`; this shows it as the menu's code
//! does (`Create` `007c0a40` writes the texts, `DoIdle` `007c2c70` and
//! `DoClick` `007c2460` the numbers). Tile ids (`AttachTileByID`
//! `007c0810`, 0 to 9):
//!
//! * 0 the bet: `_Value` `sCurrentBetText` "Current Bet: ", `_x` the bet;
//! * 1 the chips: `_Value` `sChipCountText` "Chips: ", `_x` the menu's
//!   chips;
//! * 2 the casino: `_Value` the earnings line (`world::casino::
//!   earnings_line`), its child `SMM_TotalEarningsValue` hidden;
//! * 3 to 8 the buttons' `string`s: `sSpinText` "Spin", `sIncreaseBetText`,
//!   `sDecreaseBetText`, `sPayoutListText`, `sBetMax`, `sExit` (W, E, Q, F,
//!   S, R by the menu's `_PCButton_` traits; never dimmed);
//! * 9 the status line: `sSlotPressAnyButtonText` at first (hidden), then a
//!   round's result or the payout card's prompt.
//!
//! The numbers go in as 1 and then the number (`00a01290`, `00700320`), so
//! a text copying them sees them change.

use std::collections::HashMap;

use crate::menu::MenuCode;
use crate::names::t;
use crate::tile::{TileId, Ui};
use world::casino::slots::{tile as id, SlotMachine};

/// The menu's file.
pub const FILE: &str = "menus\\slot_machine_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1080;
/// How many tile ids it keeps.
pub const TILE_COUNT: usize = 10;

/// The menu's tiles and what was done to them.
#[derive(Debug)]
pub struct SlotsMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; TILE_COUNT],
    /// Clicks for the machine, oldest first.
    pub clicks: Vec<i32>,
    /// The numbers written, by tile id (written again only when they
    /// change).
    numbers: HashMap<i32, i64>,
    texts: HashMap<i32, String>,
}

impl SlotsMenu {
    pub fn new(menu: TileId) -> SlotsMenu {
        SlotsMenu {
            menu,
            tiles: [None; TILE_COUNT],
            clicks: Vec::new(),
            numbers: HashMap::new(),
            texts: HashMap::new(),
        }
    }

    pub fn tile(&self, id: i32) -> Option<TileId> {
        usize::try_from(id)
            .ok()
            .and_then(|i| self.tiles.get(i).copied().flatten())
    }

    fn setting(ui: &Ui, name: &str) -> String {
        ui.setting_text(name).unwrap_or_default()
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

    /// Shows the menu with `Create`'s texts.
    pub fn open(&mut self, ui: &mut Ui) {
        let bet = Self::setting(ui, "sCurrentBetText");
        self.set_value_text(ui, id::CURRENT_BET, &bet);
        let chips = Self::setting(ui, "sChipCountText");
        self.set_value_text(ui, id::CHIP_COUNT, &chips);
        if let Some(info) = self.tile(id::CASINO_INFO) {
            if let Some(v) = ui.find_below(info, "SMM_TotalEarningsValue") {
                ui.set_number(v, t::VISIBLE, 0.0);
            }
        }
        for (tile, setting) in [
            (id::SPIN, "sSpinText"),
            (id::INCREASE_BET, "sIncreaseBetText"),
            (id::DECREASE_BET, "sDecreaseBetText"),
            (id::PAYOUT_LIST, "sPayoutListText"),
            (id::BET_MAX, "sBetMax"),
            (id::EXIT, "sExit"),
            (id::STATUS, "sSlotPressAnyButtonText"),
        ] {
            let s = Self::setting(ui, setting);
            if let Some(tile) = self.tile(tile) {
                ui.set_text(tile, t::STRING, &s);
            }
        }
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
    }

    /// The tiles for the machine as it is now: the bet, the chips, the
    /// earnings line, the status line (`status`: its words, worked out by
    /// the caller from [`world::casino::slots::Status`]).
    pub fn fill(&mut self, ui: &mut Ui, m: &SlotMachine, earnings: &str, status: Option<&str>) {
        self.set_x(ui, id::CURRENT_BET, i64::from(m.bet));
        self.set_x(ui, id::CHIP_COUNT, i64::from(m.chips));
        self.set_value_text(ui, id::CASINO_INFO, earnings);
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

impl MenuCode for SlotsMenu {
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
    use world::casino::CasinoData;

    #[test]
    fn texts_numbers_and_clicks() {
        let (mut ui, mut menu) = test_support::slots_menu();
        menu.open(&mut ui);
        let (m, _) =
            SlotMachine::open(&[2; 7], 9000, 1, 60, 42, 5.0, CasinoData::default()).unwrap();
        menu.fill(&mut ui, &m, "Gommorah Earnings: 0", None);
        let value = ui.names.lookup("_Value").unwrap();
        let x = ui.names.lookup("_x").unwrap();
        let bet = menu.tile(id::CURRENT_BET).unwrap();
        assert_eq!(ui.string(bet, value).as_deref(), Some("Current Bet: "));
        assert_eq!(ui.number(bet, x), 1.0);
        let chips = menu.tile(id::CHIP_COUNT).unwrap();
        assert_eq!(ui.number(chips, x), 42.0);
        let spin = menu.tile(id::SPIN).unwrap();
        assert_eq!(ui.string(spin, t::STRING).as_deref(), Some("Spin"));
        let status = menu.tile(id::STATUS).unwrap();
        assert_eq!(ui.number(status, t::VISIBLE), 0.0);
        // W spins (the menu's `_PCButton_W`).
        let mut interface = crate::menu::Interface::default();
        interface.key(
            &mut ui,
            menu.menu,
            &mut menu,
            u32::from(b'w'),
            false,
            false,
            0.0,
        );
        assert_eq!(menu.clicks, [id::SPIN]);
    }
}

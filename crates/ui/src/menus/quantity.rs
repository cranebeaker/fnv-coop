//! "How many?" (`menus\quantity_menu.xml`, class `QuantityMenu` 1016,
//! vtable `010701c4` in FalloutNV.exe), asked when moving more than
//! `iInventoryAskQuantityAt` of something. Read from the code:
//!
//! * opening (`007aba00(most, callback, start)`): `UIPopUpMessageWindow`
//!   plays; on the meter (`QM_AmountMeter`, id 0) `user0` = the most,
//!   `user2` = min(start, most), `user3` = max(most / 20, 1), cut down to a
//!   multiple of 10 above 10 (a page), `user4` = max(most / 4, 1) (a
//!   quarter), `dragx` 0; `QM_HowManyText` = `sHowMany`, the OK button (id
//!   4) `sOk`, Cancel (id 5) `sCancel`; with another menu open the
//!   background (`QM_Background`) is `fPopUpBackgroundOpacity` × 255
//!   opaque;
//! * the amount is the number `QM_AmountChosen` (id 3) shows: the file
//!   works it out from the meter's `_Value` (dragging it, `dragx`, or
//!   `user2` / `user0`);
//! * clicks (`007abe40`): the arrows (ids 1, 2) set `user2` to the amount
//!   ∓ 1 within 0..most and `dragx` back to 0; OK (4) answers the amount
//!   (truncated), Cancel (5) 0, and the menu closes (`007abdf0`);
//! * the page keys (`007abf80`, specials 15 / 16) move by a page, the
//!   triggers (13 / 14, a controller's) by a quarter, the same way;
//! * every frame (`007ac170`) a change of a page or more since the last
//!   tick plays `UIMenuFocus` (sound 4).

use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\quantity_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1016;
/// The sound it opens with.
pub const OPEN_SOUND: &str = "UIPopUpMessageWindow";

/// `user0`, `user2`, `user3`, `user4` on the meter.
const MOST: i32 = 4100;
const START: i32 = 4102;
const PAGE: i32 = 4103;
const QUARTER: i32 = 4104;

/// The quantity menu.
#[derive(Debug)]
pub struct QuantityMenu {
    pub menu: TileId,
    /// `SetTile` ids 0..5 (`007a8ab0`): `QM_AmountMeter`,
    /// `QM_DecreaseArrow`, `QM_IncreaseArrow`, `QM_AmountChosen`,
    /// `QM_OKButton`, `QM_CancelButton`.
    pub tiles: [Option<TileId>; 6],
    /// The amount when a tick last sounded (`+0x40`).
    last_tick: f32,
    /// The answer once chosen: the amount, or 0 for Cancel.
    pub answer: Option<i32>,
    /// Sounds for the caller to play (editor IDs).
    pub sounds: Vec<String>,
    pub closed: bool,
}

impl QuantityMenu {
    pub fn new(menu: TileId) -> QuantityMenu {
        QuantityMenu {
            menu,
            tiles: [None; 6],
            // The constructor (`00720a10`) leaves `+0x40` as it is; taken
            // as 0.
            last_tick: 0.0,
            answer: None,
            sounds: Vec::new(),
            closed: false,
        }
    }

    /// Opens it (`007aba00`) for up to `most`, starting at `start`;
    /// `popup` is `fPopUpBackgroundOpacity` when another menu is open.
    pub fn open(&mut self, ui: &mut Ui, most: i32, start: i32, popup: Option<f32>) {
        self.sounds.push(OPEN_SOUND.to_string());
        let mut page = (most / 20).max(1);
        if page > 10 {
            page -= page % 10;
        }
        let quarter = (most / 4).max(1);
        if let Some(meter) = self.tiles[0] {
            ui.set_number(meter, MOST, most as f32);
            ui.set_number(meter, START, start.min(most) as f32);
            ui.set_number(meter, PAGE, page as f32);
            ui.set_number(meter, QUARTER, quarter as f32);
            ui.set_number(meter, menu::drag::X, 0.0);
        }
        let menu = self.menu;
        let text = |ui: &Ui, name: &str| ui.setting_text(name).unwrap_or_default();
        if let Some(how) = ui.find(menu, "QM_HowManyText") {
            let s = text(ui, "sHowMany");
            ui.set_string(how, t::STRING, &s);
        }
        if let Some(ok) = self.tiles[4] {
            let s = text(ui, "sOk");
            ui.set_string(ok, t::STRING, &s);
        }
        if let Some(cancel) = self.tiles[5] {
            let s = text(ui, "sCancel");
            ui.set_string(cancel, t::STRING, &s);
        }
        if let (Some(opacity), Some(bg)) = (popup, ui.find(menu, "QM_Background")) {
            ui.set_number(bg, t::ALPHA, opacity * 255.0);
        }
        ui.set_number(menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(menu, t::VISIBLE, 1.0);
        ui.refresh();
    }

    /// The amount shown (`QM_AmountChosen`'s text as a number).
    pub fn amount(&self, ui: &mut Ui) -> f32 {
        let Some(chosen) = self.tiles[3] else {
            return 0.0;
        };
        ui.number(chosen, t::STRING)
    }

    /// The amount moved by `by`, kept within 0..most (`007abe40`,
    /// `007abf80`).
    fn step(&mut self, ui: &mut Ui, by: f32) {
        let Some(meter) = self.tiles[0] else {
            return;
        };
        let most = ui.number(meter, MOST);
        let v = (self.amount(ui) + by).max(0.0).min(most);
        ui.set_number(meter, START, v);
        ui.set_number(meter, menu::drag::X, 0.0);
        ui.refresh();
    }

    fn close(&mut self, ui: &mut Ui) {
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }

    /// Every frame (`007ac170`): a tick for each page moved.
    pub fn update(&mut self, ui: &mut Ui) {
        let Some(meter) = self.tiles[0] else {
            return;
        };
        let amount = self.amount(ui);
        if (amount - self.last_tick).abs() >= ui.number(meter, PAGE) {
            if let Some(s) = menu::menu_sound(4) {
                self.sounds.push(s.to_string());
            }
            self.last_tick = amount;
        }
    }
}

impl MenuCode for QuantityMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..6).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    fn click(&mut self, ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        match id {
            1 => self.step(ui, -1.0),
            2 => self.step(ui, 1.0),
            4 => {
                // `00406d90`: the float truncated to a whole number.
                self.answer = Some(self.amount(ui) as i32);
                self.close(ui);
            }
            5 => {
                self.answer = Some(0);
                self.close(ui);
            }
            _ => {}
        }
    }

    fn special_key(&mut self, ui: &mut Ui, code: i32, _now: f64) -> bool {
        let Some(meter) = self.tiles[0] else {
            return false;
        };
        let (trait_id, sign) = match code {
            13 => (QUARTER, -1.0),
            14 => (QUARTER, 1.0),
            15 => (PAGE, -1.0),
            16 => (PAGE, 1.0),
            _ => return false,
        };
        let by = ui.number(meter, trait_id) * sign;
        self.step(ui, by);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    fn opened(most: i32, start: i32) -> (Ui, QuantityMenu) {
        let mut ui = test_support::ui();
        let mut m = QuantityMenu::new(0);
        m.menu = test_support::load(&mut ui, &test_support::quantity_menu(), &mut m);
        m.open(&mut ui, most, start, None);
        (ui, m)
    }

    /// `007aba00`: the meter's values; the amount shown is the start.
    #[test]
    fn opening_sets_the_meter() {
        let (mut ui, m) = opened(250, 300);
        let meter = m.tiles[0].unwrap();
        assert_eq!(ui.number(meter, MOST), 250.0);
        assert_eq!(ui.number(meter, START), 250.0);
        // 250 / 20 = 12, cut to 10; 250 / 4 = 62.
        assert_eq!(ui.number(meter, PAGE), 10.0);
        assert_eq!(ui.number(meter, QUARTER), 62.0);
        let (mut ui2, m2) = opened(7, 3);
        let meter2 = m2.tiles[0].unwrap();
        assert_eq!(ui2.number(meter2, PAGE), 1.0);
        assert_eq!(ui2.number(meter2, QUARTER), 1.0);
        assert_eq!(m.amount(&mut ui), 250.0);
        assert_eq!(m.sounds, [OPEN_SOUND]);
        let ok = m.tiles[4].unwrap();
        assert_eq!(ui.string(ok, t::STRING).as_deref(), Some("Ok"));
    }

    /// `007abe40`, `007abf80`: the arrows and page keys, kept in range; OK
    /// answers the amount, Cancel 0.
    #[test]
    fn choosing_an_amount() {
        let (mut ui, mut m) = opened(30, 5);
        m.click(&mut ui, 2, None, 0.0);
        assert_eq!(m.amount(&mut ui), 6.0);
        m.click(&mut ui, 1, None, 0.0);
        m.click(&mut ui, 1, None, 0.0);
        assert_eq!(m.amount(&mut ui), 4.0);
        // A page is 30 / 20 = 1; a quarter 7.
        assert!(m.special_key(&mut ui, 16, 0.0));
        assert_eq!(m.amount(&mut ui), 5.0);
        assert!(m.special_key(&mut ui, 14, 0.0));
        assert_eq!(m.amount(&mut ui), 12.0);
        for _ in 0..10 {
            m.special_key(&mut ui, 13, 0.0);
        }
        assert_eq!(m.amount(&mut ui), 0.0);
        m.click(&mut ui, 2, None, 0.0);
        m.click(&mut ui, 4, None, 0.0);
        assert_eq!(m.answer, Some(1));
        assert!(m.closed);
        let (mut ui, mut m) = opened(30, 5);
        m.click(&mut ui, 5, None, 0.0);
        assert_eq!(m.answer, Some(0));
    }

    /// The meter dragged (`dragx`): the file turns the pointer into a share
    /// of the most; each page moved ticks (`007ac170`).
    #[test]
    fn dragging_the_meter_and_ticking() {
        let (mut ui, mut m) = opened(100, 0);
        m.update(&mut ui);
        assert_eq!(m.sounds.len(), 1);
        let meter = m.tiles[0].unwrap();
        // Half way along: (dragx - 140) / 392.5 = 0.5.
        ui.set_number(meter, menu::drag::X, 140.0 + 392.5 / 2.0);
        ui.refresh();
        assert_eq!(m.amount(&mut ui), 50.0);
        m.update(&mut ui);
        assert_eq!(m.sounds.last().map(String::as_str), Some("UIMenuFocus"));
        // Less than a page (5) since: no tick.
        ui.set_number(meter, menu::drag::X, 140.0 + 392.5 * 0.52);
        ui.refresh();
        let before = m.sounds.len();
        m.update(&mut ui);
        assert_eq!(m.sounds.len(), before);
    }
}

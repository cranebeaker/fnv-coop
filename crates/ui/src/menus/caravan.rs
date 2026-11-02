//! The Caravan menu's tiles (`menus\caravan_menu.xml`, whose four
//! prefabs are the screens; class `CaravanMenu` 1083, vtable `0107108c` in
//! FalloutNV.exe). The game runs in `world::caravan::menu::Menu`; this shows
//! it as the menu's code does. Tile ids (`AttachTileByID` `0073bb40`: ids 0
//! to 50 at `+0x30 + 4 × id`):
//!
//! * the screens' rects: 0 the ante, 1 the deck, 2 the game, 3 the results,
//!   each shown alone by its `Prepare…` (the results' by
//!   `PrepareResultsMenu`, as the camera starts back);
//! * the ante (`PrepareAnteMenu` `0073d020`): 4 the opponent's name and
//!   ante (its value moved right 15 a letter past 17 letters), 5 the
//!   player's ante, 6 their funds, the info rect 150 wider; titles 40, 41;
//!   buttons 37 Auto Match (W), 36 Raise (A), 7 Accept (F), 38 Exit (R);
//! * the deck (`PrepareDeckMenu` `0073d850`): 8 the cards in the deck (its
//!   first child dimmed below 30), 9 the cards owned, 42 "Caravan Deck", 43;
//!   10 "Randomize Caravan Deck" (S) and 11 "All Cards", both 150 to the
//!   left; 39 Navigate hidden; 12 Add (W), 13 Remove (A), 14 Play Caravan
//!   (F); 15 the scrollbar (`user0` the cards owned, `_current_value` the
//!   one chosen);
//! * the game (`PrepareGameMenu` `0073ea90`): 16 the ante pot (the stake ×
//!   2), 17 the net total to date (won − lost), 18 games won / lost
//!   ("%i/%i"), the info rect 150 wider and 150 left; 19 Select Card (W),
//!   20 Discard Card (Q), 21 Discard Track (E), 22 "Cancel" (R); 23–25 the
//!   player's tracks' values and 26–28 the opponent's (5 to 3), system
//!   colour 2 over 26 else 1, their rects shown and placed once the hands
//!   are dealt (state 2's end: the width of rect 2 × where the camera puts
//!   `PlayerGrid_1`'s `Select1_01:0` across the screen − 120, + 265 and +
//!   530 for the next, each track's with the one facing it); 29 and 30 the
//!   player's and opponent's draw piles;
//! * the results (`PrepareResultsMenu` `00740980`): 31 caps won to date, 32
//!   lost, 33 games won / lost, 34 the biggest pot won, 35 "You win!" /
//!   "You Lose!", then "Press Any Button to Continue"; titles 47–50; the
//!   info rect 150 wider.
//!
//! Buttons are lit (alpha 255) when they work and dimmed (128) when not or
//! while busy (`UpdateTileAlpha` `00749a40`, `SetTileTextAlpha` `0074a000`
//! on each child of the button's tile).

use crate::menu::{key, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};
use world::caravan::menu::{flag, state, Arrow, Menu, Screen};

/// The menu's file.
pub const FILE: &str = "menus\\caravan_menu.xml";
/// Its class number.
pub const CLASS: i32 = 1083;
/// How many tile ids it keeps.
pub const TILE_COUNT: usize = 51;

/// Tile ids.
pub mod id {
    pub const ANTE: i32 = 0;
    pub const DECK: i32 = 1;
    pub const GAME: i32 = 2;
    pub const RESULTS: i32 = 3;
    pub const NPC_BET: i32 = 4;
    pub const PLAYER_BET: i32 = 5;
    pub const FUNDS: i32 = 6;
    pub const ACCEPT: i32 = 7;
    pub const IN_DECK: i32 = 8;
    pub const OWNED: i32 = 9;
    pub const RANDOMIZE: i32 = 10;
    pub const ALL_CARDS: i32 = 11;
    pub const ADD: i32 = 12;
    pub const REMOVE: i32 = 13;
    pub const PLAY: i32 = 14;
    pub const SCROLLBAR: i32 = 15;
    pub const ANTE_POT: i32 = 16;
    pub const NET_TOTAL: i32 = 17;
    pub const WIN_LOSS: i32 = 18;
    pub const SELECT_CARD: i32 = 19;
    pub const DISCARD_CARD: i32 = 20;
    pub const DISCARD_TRACK: i32 = 21;
    pub const CANCEL: i32 = 22;
    /// Tracks 0–5's values: 23 + t.
    pub const TRACK_VALUES: i32 = 23;
    pub const PLAYER_PILE: i32 = 29;
    pub const NPC_PILE: i32 = 30;
    pub const WINNINGS: i32 = 31;
    pub const LOSSES: i32 = 32;
    pub const RECORD: i32 = 33;
    pub const BIGGEST: i32 = 34;
    pub const INFO: i32 = 35;
    pub const RAISE: i32 = 36;
    pub const AUTO_MATCH: i32 = 37;
    pub const EXIT: i32 = 38;
    pub const NAVIGATE: i32 = 39;
}

/// What the tiles show besides the game itself.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Facts {
    /// The opponent's name.
    pub npc_name: String,
    /// The player's record now (caps won and lost, games won and lost, the
    /// biggest pot).
    pub cap_winnings: u32,
    pub cap_losses: u32,
    pub winnings: u32,
    pub losses: u32,
    pub largest_winning: u32,
}

/// The menu's tiles and what was done to them.
#[derive(Debug)]
pub struct CaravanMenu {
    pub menu: TileId,
    pub tiles: [Option<TileId>; TILE_COUNT],
    /// Clicks, typed keys and arrows for the game, oldest first.
    pub clicks: Vec<i32>,
    pub typed: Vec<char>,
    pub arrows: Vec<Arrow>,
    /// The screens whose layout was set up (once each).
    prepared: [bool; 4],
    /// The `_Value`s written, by tile id (written again only when they
    /// change).
    values: std::collections::HashMap<i32, i64>,
    /// The track values placed.
    pub tracks_placed: bool,
}

impl CaravanMenu {
    pub fn new(menu: TileId) -> CaravanMenu {
        CaravanMenu {
            menu,
            tiles: [None; TILE_COUNT],
            clicks: Vec::new(),
            typed: Vec::new(),
            arrows: Vec::new(),
            prepared: [false; 4],
            values: std::collections::HashMap::new(),
            tracks_placed: false,
        }
    }

    pub fn tile(&self, id: i32) -> Option<TileId> {
        usize::try_from(id)
            .ok()
            .and_then(|i| self.tiles.get(i).copied().flatten())
    }

    fn text(ui: &Ui, setting: &str) -> String {
        ui.setting_text(setting).unwrap_or_default()
    }

    fn set_string(&self, ui: &mut Ui, id: i32, s: &str) {
        if let Some(tile) = self.tile(id) {
            ui.set_text(tile, t::STRING, s);
        }
    }

    fn set_setting(&self, ui: &mut Ui, id: i32, setting: &str) {
        let s = Self::text(ui, setting);
        self.set_string(ui, id, &s);
    }

    /// `_Value` (written as 1 first, then the number: `00a01290`,
    /// `00700320`; the game works the tiles out after each, so a text
    /// copying it sees it change even to the number it had).
    fn set_value(&mut self, ui: &mut Ui, id: i32, v: i64) {
        if self.values.get(&id) == Some(&v) {
            return;
        }
        if let Some(tile) = self.tile(id) {
            self.values.insert(id, v);
            let value = ui.names.lookup_or_add("_Value").unwrap_or(0);
            ui.set_number(tile, value, 1.0);
            ui.work_out_all(tile);
            ui.set_number(tile, value, v as f32);
            ui.work_out_all(tile);
        }
    }

    fn set_value_text(&self, ui: &mut Ui, id: i32, s: &str) {
        if let Some(tile) = self.tile(id) {
            let value = ui.names.lookup_or_add("_Value").unwrap_or(0);
            ui.set_number(tile, value, 1.0);
            ui.set_text(tile, value, s);
        }
    }

    fn shift(&self, ui: &mut Ui, tile: Option<TileId>, trait_id: i32, by: f32) {
        if let Some(tile) = tile {
            let v = ui.number(tile, trait_id);
            ui.set_number(tile, trait_id, v + by);
        }
    }

    /// A button's children lit or dimmed (`SetTileTextAlpha`).
    fn button_alpha(&self, ui: &mut Ui, id: i32, lit: bool) {
        let Some(tile) = self.tile(id) else {
            return;
        };
        let alpha = if lit { 255.0 } else { 128.0 };
        let children = ui.tiles[tile].children.clone();
        for child in children {
            ui.set_number(child, t::ALPHA, alpha);
        }
    }

    /// Shows the menu (`Create`): the ante's texts.
    pub fn open(&mut self, ui: &mut Ui) {
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
    }

    /// The tiles for the menu as it is now: which screen, the numbers, the
    /// buttons. `track_x`: where the camera puts `PlayerGrid_1`'s
    /// `Select1_01:0` across the screen (0 to 1), once the hands are dealt.
    pub fn fill(&mut self, ui: &mut Ui, m: &Menu, facts: &Facts, track_x: Option<f32>) {
        let results = m.screen == Screen::Results || m.state == state::CAMERA_TO_RESULTS;
        let shown = if results { 3 } else { m.screen as usize };
        for i in 0..4 {
            if let Some(tile) = self.tile(i as i32) {
                ui.set_number(tile, t::VISIBLE, if i == shown { 1.0 } else { 0.0 });
            }
        }
        if !self.prepared[shown] {
            self.prepared[shown] = true;
            self.prepare(ui, shown, m, facts);
        }
        match shown {
            0 => {
                self.set_value(ui, id::NPC_BET, i64::from(m.bet.npc_ante));
                self.set_value(ui, id::PLAYER_BET, i64::from(m.bet.player_ante));
                self.set_value(ui, id::FUNDS, i64::from(m.bet.player_funds));
                self.button_alpha(ui, id::ACCEPT, m.lit(flag::DISABLE_Y));
                self.button_alpha(ui, id::AUTO_MATCH, m.lit(flag::DISABLE_A));
                self.button_alpha(ui, id::RAISE, m.lit(flag::DISABLE_X));
            }
            1 => {
                self.set_value(ui, id::IN_DECK, m.in_deck as i64);
                self.set_value(ui, id::OWNED, m.cards.len() as i64);
                if let Some(bar) = self.tile(id::SCROLLBAR) {
                    ui.set_number(bar, t::USER0, m.cards.len() as f32);
                    // Set from the code only when the code moved it
                    // (`_SetInCode`), so a drag isn't undone.
                    if self.values.get(&id::SCROLLBAR) != Some(&(m.chosen as i64)) {
                        self.values.insert(id::SCROLLBAR, m.chosen as i64);
                        let current = ui.names.lookup_or_add("_current_value").unwrap_or(0);
                        // Its operators kept (the arrows, wheel and drag
                        // add to it).
                        ui.set_base(bar, current, m.chosen as f32);
                    }
                }
                // Below 30 the count's first child is dimmed.
                if let Some(first) = self
                    .tile(id::IN_DECK)
                    .and_then(|tile| ui.tiles[tile].children.first().copied())
                {
                    let enough = m.in_deck >= world::caravan::MIN_DECK;
                    ui.set_number(first, t::ALPHA, if enough { 255.0 } else { 128.0 });
                }
                self.button_alpha(ui, id::RANDOMIZE, m.lit(flag::DISABLE_RB));
                self.button_alpha(ui, id::ADD, m.lit(flag::DISABLE_A));
                self.button_alpha(ui, id::REMOVE, m.lit(flag::DISABLE_X));
                self.button_alpha(ui, id::PLAY, m.lit(flag::DISABLE_Y));
                self.button_alpha(ui, id::OWNED, true);
                self.button_alpha(ui, id::ALL_CARDS, true);
            }
            2 => {
                if let Some(g) = &m.game {
                    for (t_, info) in g.info.iter().enumerate() {
                        let tile_id = id::TRACK_VALUES + t_ as i32;
                        self.set_value(ui, tile_id, i64::from(info.value));
                        if let Some(tile) = self.tile(tile_id) {
                            let colour = if info.value > 26 { 2.0 } else { 1.0 };
                            ui.set_number(tile, t::SYSTEMCOLOR, colour);
                            ui.set_number(tile, t::ALPHA, 255.0);
                        }
                    }
                }
                self.set_value(ui, id::PLAYER_PILE, m.piles[0] as i64);
                self.set_value(ui, id::NPC_PILE, m.piles[1] as i64);
                self.set_value(ui, id::ANTE_POT, i64::from(m.stake) * 2);
                self.button_alpha(ui, id::SELECT_CARD, m.lit(flag::DISABLE_A));
                self.button_alpha(ui, id::DISCARD_CARD, m.lit(flag::DISABLE_LT));
                self.button_alpha(ui, id::DISCARD_TRACK, m.lit(flag::DISABLE_RT));
                self.button_alpha(ui, id::CANCEL, m.lit(flag::DISABLE_B));
                if let (false, Some(x)) = (self.tracks_placed, track_x) {
                    self.place_tracks(ui, x);
                }
            }
            _ => {
                self.set_value(ui, id::WINNINGS, i64::from(facts.cap_winnings));
                self.set_value(ui, id::LOSSES, i64::from(facts.cap_losses));
                let record = format!("{}/{}", facts.winnings, facts.losses);
                self.set_value_text(ui, id::RECORD, &record);
                self.set_value(ui, id::BIGGEST, i64::from(facts.largest_winning));
                let info = if m.screen == Screen::Results {
                    "sCaravanPressAnyKeyText"
                } else if m.has(flag::PLAYER_WIN) {
                    "sCaravanYouWinText"
                } else {
                    "sCaravanYouLoseText"
                };
                self.set_setting(ui, id::INFO, info);
            }
        }
        ui.refresh();
    }

    /// A screen's one-time setup (its `Prepare…`).
    fn prepare(&mut self, ui: &mut Ui, screen: usize, m: &Menu, facts: &Facts) {
        match screen {
            0 => {
                self.set_string(ui, id::NPC_BET, &facts.npc_name);
                for (tile_id, setting) in [
                    (40, "sCurrentAnteText"),
                    (41, "sTotalFundsText"),
                    (id::AUTO_MATCH, "sAutoMatchText"),
                    (id::RAISE, "sRaiseText"),
                    (id::ACCEPT, "sAcceptText"),
                    (id::EXIT, "sExit"),
                ] {
                    self.set_setting(ui, tile_id, setting);
                }
                let letters = facts.npc_name.chars().count();
                if letters > 17 {
                    let value = ui.find(self.menu, "CBM_NPCBetValue");
                    self.shift(ui, value, t::X, (letters - 17) as f32 * 15.0);
                }
                let info = ui.find(self.menu, "CBM_InfoRect");
                self.shift(ui, info, t::WIDTH, 150.0);
            }
            1 => {
                let randomize = format!(
                    "{} {}",
                    Self::text(ui, "sRSMRandomize"),
                    Self::text(ui, "sCaravanDeckText")
                );
                self.set_string(ui, id::RANDOMIZE, &randomize);
                if let Some(tile) = self.tile(id::RANDOMIZE) {
                    let hint = ui.names.lookup_or_add("_PCButtonText").unwrap_or(0);
                    let s = Self::text(ui, "sPCMenuHintS");
                    ui.set_text(tile, hint, &s);
                }
                for (tile_id, setting) in [
                    (id::ALL_CARDS, "sAllCardsText"),
                    (id::NAVIGATE, "sNavigateText"),
                    (id::ADD, "sAddText"),
                    (id::REMOVE, "sRemoveText"),
                    (id::PLAY, "sPlayCaravanText"),
                    (42, "sCaravanDeckText"),
                    (43, "sTotalCardsText"),
                ] {
                    self.set_setting(ui, tile_id, setting);
                }
                for tile_id in [id::RANDOMIZE, id::ALL_CARDS] {
                    let tile = self.tile(tile_id);
                    self.shift(ui, tile, t::X, -150.0);
                }
                if let Some(tile) = self.tile(id::NAVIGATE) {
                    ui.set_number(tile, t::VISIBLE, 0.0);
                }
            }
            2 => {
                for (tile_id, setting) in [
                    (44, "sAntePotText"),
                    (45, "sNetTotalText"),
                    (46, "sWinLossText"),
                    (id::SELECT_CARD, "sSelectCardText"),
                    (id::DISCARD_CARD, "sDiscardCardText"),
                    (id::DISCARD_TRACK, "sDiscardTrackText"),
                    (id::CANCEL, "sCancel"),
                ] {
                    self.set_setting(ui, tile_id, setting);
                }
                let net = i64::from(facts.cap_winnings) - i64::from(facts.cap_losses);
                self.set_value(ui, id::NET_TOTAL, net);
                let record = format!("{}/{}", facts.winnings, facts.losses);
                self.set_value_text(ui, id::WIN_LOSS, &record);
                let info = ui.find(self.menu, "CGM_InfoRect");
                self.shift(ui, info, t::WIDTH, 150.0);
                self.shift(ui, info, t::X, -150.0);
                let _ = m;
            }
            _ => {
                for (tile_id, setting) in [
                    (47, "sCaravanWinningsText"),
                    (48, "sCaravanLossesText"),
                    (49, "sCaravanWinLossText"),
                    (50, "sCaravanBiggestAnteText"),
                ] {
                    self.set_setting(ui, tile_id, setting);
                }
                let info = ui.find(self.menu, "CRM_InfoRect");
                self.shift(ui, info, t::WIDTH, 150.0);
            }
        }
    }

    /// Where the deck screen's scrollbar is (its `_current_value`, which its
    /// prefab's arrows, wheel and marker drag move; `DoIdle` reads it as a
    /// whole number).
    pub fn meter(&mut self, ui: &mut Ui) -> Option<usize> {
        let bar = self.tile(id::SCROLLBAR)?;
        let current = ui.names.lookup_or_add("_current_value").unwrap_or(0);
        Some(ui.number(bar, current).max(0.0) as usize)
    }

    /// The scrollbar's value taken as the code's own (so it isn't set back
    /// while the cards move to it).
    pub fn meter_taken(&mut self, at: usize) {
        self.values.insert(id::SCROLLBAR, at as i64);
    }

    /// The track values' places (state 2's end, `00741d4f`): x = the width
    /// of rect 2 × `x` − 120 for tracks 0 and 5, + 265 for 1 and 4, + 530
    /// for 2 and 3; their rects shown.
    pub fn place_tracks(&mut self, ui: &mut Ui, x: f32) {
        self.tracks_placed = true;
        let width = self.tile(id::GAME).map_or(0.0, |g| ui.number(g, t::WIDTH));
        let base = (width as i32) as f32 * x - 120.0;
        for (track, offset) in [
            (0, 0.0),
            (5, 0.0),
            (1, 265.0),
            (4, 265.0),
            (2, 530.0),
            (3, 530.0),
        ] {
            if let Some(tile) = self.tile(id::TRACK_VALUES + track) {
                ui.set_number(tile, t::X, base + offset);
            }
        }
        for name in ["CGM_NPCTrackRect", "CGM_PlayerTrackRect"] {
            if let Some(rect) = ui.find(self.menu, name) {
                ui.set_number(rect, t::VISIBLE, 1.0);
            }
        }
        ui.refresh();
    }
}

impl MenuCode for CaravanMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if (0..TILE_COUNT as i32).contains(&id) {
            self.tiles[id as usize] = Some(tile);
        }
    }

    /// `DoClick` (`00749560`): passed on to the game.
    fn click(&mut self, _ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        self.clicks.push(id);
    }

    /// `00749360`: letters and the arrows passed on.
    fn key(&mut self, _ui: &mut Ui, code: u32, _now: f64) -> bool {
        let arrow = match code {
            key::LEFT => Some(Arrow::Left),
            key::RIGHT => Some(Arrow::Right),
            key::UP => Some(Arrow::Up),
            key::DOWN => Some(Arrow::Down),
            _ => None,
        };
        if let Some(a) = arrow {
            self.arrows.push(a);
            return true;
        }
        match char::from_u32(code) {
            Some(c) if code < 0x8000_0000 => {
                self.typed.push(c);
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;
    use esm::FormId;
    use world::caravan::bet::Bet;
    use world::caravan::Card;

    fn cards(n: u32, base: u32) -> Vec<Card> {
        (0..n)
            .map(|i| Card {
                form: FormId(base + i),
                value: (i % 10) as i32 + 1,
                suit: (i % 4) as i32 + 1,
            })
            .collect()
    }

    fn value(ui: &mut Ui, m: &CaravanMenu, id: i32) -> f32 {
        let tile = m.tile(id).unwrap();
        let v = ui.names.lookup_or_add("_Value").unwrap();
        ui.number(tile, v)
    }

    fn child_alpha(ui: &mut Ui, m: &CaravanMenu, id: i32) -> f32 {
        let tile = m.tile(id).unwrap();
        let child = ui.tiles[tile].children[0];
        ui.number(child, t::ALPHA)
    }

    /// The ante's tiles, then the deck screen's, as the menu moves on.
    #[test]
    fn ante_and_deck_tiles() {
        let mut ui = test_support::ui();
        let mut code = CaravanMenu::new(0);
        let menu = test_support::load(&mut ui, &test_support::caravan_menu(), &mut code);
        code.menu = menu;
        let (mut m, _) = Menu::open(
            FormId(0x50),
            cards(30, 0x300),
            1,
            &cards(4, 0x100),
            &cards(30, 0x200),
            Bet::new(250, 200, 0.5),
            0.0,
            &mut |_| 0,
        );
        let facts = Facts {
            npc_name: "A Very Long Opponent Name".into(),
            ..Facts::default()
        };
        code.open(&mut ui);
        code.fill(&mut ui, &m, &facts, None);
        let shown = |ui: &mut Ui, code: &CaravanMenu, id: i32| {
            let tile = code.tile(id).unwrap();
            ui.number(tile, t::VISIBLE)
        };
        assert_eq!(shown(&mut ui, &code, 0), 1.0);
        assert_eq!(shown(&mut ui, &code, 1), 0.0);
        assert_eq!(value(&mut ui, &code, id::NPC_BET), 100.0);
        assert_eq!(value(&mut ui, &code, id::FUNDS), 250.0);
        // Accept dimmed (nothing put in yet), Auto Match and Raise lit,
        // unless busy.
        m.flags &= !flag::DISABLE_ALL;
        code.fill(&mut ui, &m, &facts, None);
        assert_eq!(child_alpha(&mut ui, &code, id::ACCEPT), 128.0);
        assert_eq!(child_alpha(&mut ui, &code, id::AUTO_MATCH), 255.0);
        // The long name pushes its value right 15 a letter past 17.
        let v = ui.find(code.menu, "CBM_NPCBetValue").unwrap();
        assert_eq!(ui.number(v, t::X), 20.0 + 8.0 * 15.0);
        let info = ui.find(code.menu, "CBM_InfoRect").unwrap();
        assert_eq!(ui.number(info, t::WIDTH), 450.0);
        // The deck screen.
        m.screen = Screen::Deck;
        m.update_flags();
        code.fill(&mut ui, &m, &facts, None);
        assert_eq!(shown(&mut ui, &code, 0), 0.0);
        assert_eq!(shown(&mut ui, &code, 1), 1.0);
        assert_eq!(value(&mut ui, &code, id::IN_DECK), 30.0);
        assert_eq!(value(&mut ui, &code, id::OWNED), 34.0);
        let label = ui.string(code.tile(id::RANDOMIZE).unwrap(), t::STRING);
        assert_eq!(label.as_deref(), Some("Randomize Caravan Deck"));
        let nav = code.tile(id::NAVIGATE).unwrap();
        assert_eq!(ui.number(nav, t::VISIBLE), 0.0);
    }

    /// Keys and clicks wait for the game.
    #[test]
    fn input_is_passed_on() {
        let mut ui = test_support::ui();
        let mut code = CaravanMenu::new(0);
        test_support::load(&mut ui, &test_support::caravan_menu(), &mut code);
        assert!(code.key(&mut ui, 'w' as u32, 0.0));
        assert!(code.key(&mut ui, key::LEFT, 0.0));
        code.click(&mut ui, id::ACCEPT, None, 0.0);
        assert_eq!(code.typed, vec!['w']);
        assert_eq!(code.arrows, vec![Arrow::Left]);
        assert_eq!(code.clicks, vec![id::ACCEPT]);
    }
}

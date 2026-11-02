//! The tutorial message box (`menus\tutorial_menu.xml`, class
//! `TutorialMenu` 1059, vtable `0106ff84` in FalloutNV.exe): a help
//! message (`world::tutorial`) shown once, over the menu it's about. Read
//! from the code (names from the Xbox 360 prototype's PDB):
//!
//! * opening (`TutorialMenu::Create`, `007e8890`, a message and whether
//!   it's the help manual): a tutorial menu already open is closed; the
//!   menu file loaded; `TM_VDSG_text` (id 7) gets `sVDSGManual`. With one
//!   message (every caller but the start menu's Help): Next (1) and
//!   Previous (2) are hidden. As the help manual (the form list
//!   `HelpManual`, 0x163; `HelpManualXBox` 0x165 with a pad): Next and
//!   Previous get `sNext` / `sPrevious`, and no message means the
//!   manual's first. Close (3) gets `sCloseButton`. The pages are the
//!   manual's messages: the message's page is shown "n/N" on the page
//!   number (5); one not in the manual (and every single message) has no
//!   pages, the page number hidden (its place taken by the file's
//!   `TM_fill_line`). With no message at all the menu closes again
//!   ("Warning:  Unable to find valid starting message for Tutorial
//!   Menu."). Then the message (`SetTitleAndText`, `007e9060`): the title
//!   (4) its `FULL` name, the text (0) its `DESC` (an HTML text when it
//!   starts with `<`), the scrollbar (6) back to the top
//!   (`_current_value` 0, its operators kept); then Next and Previous are
//!   made clickable or not by the page (`UpdatePrevNext`, `007e9010`:
//!   neither without pages); and the menu is shown;
//! * a click (`007e8db0`): Next (1) and Previous (2) turn the manual's
//!   pages (`007e8e20` / `007e8e40` → `007e8ed0`: a page past the last,
//!   or before the first, does nothing), Close (3) closes (`007e8d60`);
//!   the file's `_PCButton_E` is Close;
//! * the cancel code (10, `007e8e80`) closes it when it's the menu on top.

use crate::menu::{self, MenuCode};
use crate::names::t;
use crate::tile::{TileId, Ui};

/// The menu's file.
pub const FILE: &str = "menus\\tutorial_menu.xml";
/// Its class number (`&TutorialMenu;`).
pub const CLASS: i32 = 1059;
/// The code that closes it (`007e8e80`).
pub const CLOSE_CODE: i32 = 10;

/// The tiles by id (`007209e0`, ids 0–7).
pub mod tile {
    pub const TEXT: usize = 0;
    pub const NEXT: usize = 1;
    pub const PREVIOUS: usize = 2;
    pub const CLOSE: usize = 3;
    pub const TITLE: usize = 4;
    pub const PAGE: usize = 5;
    pub const SCROLLBAR: usize = 6;
    pub const VDSG_TEXT: usize = 7;
}

/// A help message: its record (form ID) and its `FULL` and `DESC`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Message {
    pub form: u32,
    pub title: String,
    pub text: String,
}

/// The tutorial menu.
#[derive(Debug)]
pub struct TutorialMenu {
    pub menu: TileId,
    /// `pTiles` (+0x28).
    pub tiles: [Option<TileId>; 8],
    /// The page shown and how many there are (`iCurrentPage` +0x48,
    /// `iNumPages` +0x4c): 0 and 0 with one message.
    pub page: u32,
    pub pages: u32,
    /// The help manual's messages, as pages (+0x50, the form list's
    /// entries); none with one message.
    pub manual: Vec<Message>,
    /// The tutorial id it shows, for the caller.
    pub id: u8,
    pub closed: bool,
}

impl TutorialMenu {
    /// A tutorial menu for a menu tile loaded with [`menu::load`].
    pub fn new(menu: TileId, id: u8) -> TutorialMenu {
        TutorialMenu {
            menu,
            tiles: [None; 8],
            page: 0,
            pages: 0,
            manual: Vec::new(),
            id,
            closed: false,
        }
    }

    fn tile(&self, i: usize) -> Option<TileId> {
        self.tiles.get(i).copied().flatten()
    }

    /// Opens it with one message (`007e8890(message, 0)`).
    pub fn open(&mut self, ui: &mut Ui, message: &Message) {
        self.create(ui, Some(message.clone()), None);
    }

    /// Opens it as the help manual (`007e8890(message, 1)`): `manual` the
    /// help manual's messages, `message` the one to show (none: the
    /// first). False when there's none to show (the menu closed again).
    pub fn open_manual(
        &mut self,
        ui: &mut Ui,
        message: Option<Message>,
        manual: Vec<Message>,
    ) -> bool {
        self.create(ui, message, Some(manual))
    }

    /// `TutorialMenu::Create` (`007e8890`) once the file is loaded.
    fn create(
        &mut self,
        ui: &mut Ui,
        message: Option<Message>,
        manual: Option<Vec<Message>>,
    ) -> bool {
        let text = |ui: &Ui, name: &str| ui.setting_text(name).unwrap_or_default();
        if let Some(vdsg) = self.tile(tile::VDSG_TEXT) {
            let s = text(ui, "sVDSGManual");
            ui.set_string(vdsg, t::STRING, &s);
        }
        let mut message = message;
        match manual {
            None => {
                for i in [tile::NEXT, tile::PREVIOUS] {
                    if let Some(b) = self.tile(i) {
                        ui.set_number(b, t::VISIBLE, 0.0);
                    }
                }
            }
            Some(manual) => {
                if message.is_none() {
                    message = manual.first().cloned();
                }
                self.manual = manual;
                for (i, name) in [(tile::NEXT, "sNext"), (tile::PREVIOUS, "sPrevious")] {
                    if let Some(b) = self.tile(i) {
                        let s = text(ui, name);
                        ui.set_string(b, t::STRING, &s);
                    }
                }
            }
        }
        if let Some(close) = self.tile(tile::CLOSE) {
            let s = text(ui, "sCloseButton");
            ui.set_string(close, t::STRING, &s);
        }
        // The pages: the manual's messages, the shown one's page "n/N".
        // One message, or one not in the manual: no pages.
        self.page = 0;
        self.pages = self.manual.len() as u32;
        let found = message
            .as_ref()
            .and_then(|m| self.manual.iter().position(|p| p.form == m.form));
        match found {
            Some(i) => {
                self.page = i as u32;
                self.set_page_number(ui);
            }
            None => {
                self.pages = 0;
                if let Some(page) = self.tile(tile::PAGE) {
                    ui.set_number(page, t::VISIBLE, 0.0);
                }
            }
        }
        let Some(message) = message else {
            println!("Warning:  Unable to find valid starting message for Tutorial Menu.");
            self.close(ui);
            return false;
        };
        self.set_title_and_text(ui, &message);
        self.update_prev_next(ui);
        ui.set_number(self.menu, menu::LEAVE_STACK, 0.0);
        ui.set_number(self.menu, t::VISIBLE, 1.0);
        ui.refresh();
        true
    }

    /// The page number, "n/N".
    fn set_page_number(&mut self, ui: &mut Ui) {
        if let Some(page) = self.tile(tile::PAGE) {
            let s = format!("{}/{}", self.page + 1, self.pages);
            ui.set_string(page, t::STRING, &s);
        }
    }

    /// Turns to a page of the manual (`007e8ed0`): nothing for one past
    /// the last (or before the first, which wraps round to past it).
    fn turn(&mut self, ui: &mut Ui, page: u32) {
        if page >= self.pages {
            return;
        }
        self.page = page;
        self.set_page_number(ui);
        match self.manual.get(page as usize).cloned() {
            Some(m) => {
                self.set_title_and_text(ui, &m);
                self.update_prev_next(ui);
            }
            None => println!(
                "End of Help Manual reached while searching for page {page} in help manual."
            ),
        }
        ui.refresh();
    }

    /// `SetTitleAndText` (`007e9060`).
    fn set_title_and_text(&mut self, ui: &mut Ui, message: &Message) {
        if let Some(title) = self.tile(tile::TITLE) {
            ui.set_string(title, t::STRING, &message.title);
        }
        if let Some(text) = self.tile(tile::TEXT) {
            let html = message.text.starts_with('<');
            ui.set_number(text, t::ISHTML, f32::from(html));
            ui.set_string(text, t::STRING, &message.text);
        }
        if let Some(bar) = self.tile(tile::SCROLLBAR) {
            let current = ui.names.lookup_or_add("_current_value").unwrap_or(0);
            ui.set_base(bar, current, 0.0);
        }
    }

    /// `UpdatePrevNext` (`007e9010`): Previous clickable past the first
    /// page, Next before the last.
    fn update_prev_next(&mut self, ui: &mut Ui) {
        if let Some(prev) = self.tile(tile::PREVIOUS) {
            ui.set_number(prev, t::TARGET, f32::from(self.page != 0));
        }
        if let Some(next) = self.tile(tile::NEXT) {
            ui.set_number(next, t::TARGET, f32::from(self.page + 1 < self.pages));
        }
    }

    /// Closes it (`007e8d60`).
    pub fn close(&mut self, ui: &mut Ui) {
        if self.closed {
            return;
        }
        ui.set_number(self.menu, menu::LEAVE_STACK, 1.0);
        self.closed = true;
    }
}

impl MenuCode for TutorialMenu {
    fn class(&self) -> i32 {
        CLASS
    }

    fn set_tile(&mut self, id: i32, tile: TileId) {
        if let Some(slot) = usize::try_from(id).ok().and_then(|i| self.tiles.get_mut(i)) {
            *slot = Some(tile);
        }
    }

    /// `007e8db0`: Next and Previous turn the manual's pages, Close
    /// closes.
    fn click(&mut self, ui: &mut Ui, id: i32, _tile: Option<TileId>, _now: f64) {
        match id {
            i if i == tile::NEXT as i32 => self.turn(ui, self.page.wrapping_add(1)),
            i if i == tile::PREVIOUS as i32 => self.turn(ui, self.page.wrapping_sub(1)),
            i if i == tile::CLOSE as i32 => self.close(ui),
            _ => {}
        }
    }

    /// `007e8e80`: the cancel code closes it.
    fn special_key(&mut self, ui: &mut Ui, code: i32, _now: f64) -> bool {
        if code == CLOSE_CODE {
            self.close(ui);
            return true;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menus::test_support;

    /// The parts of `tutorial_menu.xml` the code reads.
    const XML: &str = "<menu name=\"TutorialMenu\"><class>&TutorialMenu;</class>
        <_PCButton_E> TM_close_button </_PCButton_E>
        <rect name=\"TM_depth_rect\">
          <text name=\"TM_title\"><id>4</id><font>6</font><string> Title Text </string></text>
          <text name=\"TM_page\"><id>5</id><font>6</font><string> ??/?? </string></text>
          <hotrect name=\"TM_scrollbar\"><id>6</id><_current_value>5</_current_value></hotrect>
          <hotrect name=\"TM_text_container\"><width>600</width><height>400</height>
            <text name=\"TM_text\"><id>0</id><wrapwidth>595</wrapwidth></text></hotrect>
          <hotrect name=\"TM_next_button\"><id>1</id><target>&true;</target><string>&-sNext;</string></hotrect>
          <hotrect name=\"TM_prev_button\"><id>2</id><target>&true;</target><string>&-sPrevious;</string></hotrect>
          <hotrect name=\"TM_close_button\"><id>3</id><target>&true;</target><width>100</width><height>30</height>
            <string>x</string></hotrect>
          <text name=\"TM_VDSG_text\"><id>7</id><string>x</string></text>
        </rect></menu>";

    fn opened(text: &str) -> (Ui, TutorialMenu) {
        let mut ui = test_support::ui();
        let mut m = TutorialMenu::new(0, 19);
        m.menu = test_support::load(&mut ui, XML, &mut m);
        m.open(
            &mut ui,
            &Message {
                form: 0x17b,
                title: "Hacking".into(),
                text: text.into(),
            },
        );
        (ui, m)
    }

    #[test]
    fn opening_fills_it_with_the_message() {
        let (mut ui, m) = opened("Some terminals are protected.");
        let tile = |i: usize| m.tiles[i].unwrap();
        assert_eq!(ui.string(tile(tile::TITLE), t::STRING).unwrap(), "Hacking");
        assert_eq!(
            ui.string(tile(tile::TEXT), t::STRING).unwrap(),
            "Some terminals are protected."
        );
        assert_eq!(ui.number(tile(tile::TEXT), t::ISHTML), 0.0);
        assert_eq!(ui.string(tile(tile::CLOSE), t::STRING).unwrap(), "Close");
        assert_eq!(
            ui.string(tile(tile::VDSG_TEXT), t::STRING).unwrap(),
            "VDSG MANUAL"
        );
        // One message: no pages, Next and Previous hidden and not
        // clickable.
        for i in [tile::NEXT, tile::PREVIOUS, tile::PAGE] {
            assert_eq!(ui.number(tile(i), t::VISIBLE), 0.0, "tile {i}");
        }
        assert_eq!(ui.number(tile(tile::NEXT), t::TARGET), 0.0);
        assert_eq!(ui.number(tile(tile::PREVIOUS), t::TARGET), 0.0);
        let current = ui.names.lookup_or_add("_current_value").unwrap();
        assert_eq!(ui.number(tile(tile::SCROLLBAR), current), 0.0);
        assert_eq!(ui.number(m.menu, t::VISIBLE), 1.0);
        assert_eq!(ui.number(m.menu, menu::LEAVE_STACK), 0.0);
    }

    #[test]
    fn a_text_starting_with_a_tag_is_html() {
        let (mut ui, m) = opened("<p>Hi</p>");
        assert_eq!(ui.number(m.tiles[tile::TEXT].unwrap(), t::ISHTML), 1.0);
    }

    #[test]
    fn close_and_the_cancel_code_close_it() {
        let (mut ui, mut m) = opened("Text");
        m.click(&mut ui, tile::NEXT as i32, None, 0.0);
        m.click(&mut ui, tile::PREVIOUS as i32, None, 0.0);
        assert!(!m.closed);
        assert!(!m.special_key(&mut ui, crate::menu::special::A, 0.0));
        m.click(&mut ui, tile::CLOSE as i32, None, 0.0);
        assert!(m.closed);
        assert_eq!(ui.number(m.menu, menu::LEAVE_STACK), 1.0);
        let (mut ui, mut m) = opened("Text");
        assert!(m.special_key(&mut ui, CLOSE_CODE, 0.0));
        assert!(m.closed);
    }

    fn page(n: u32) -> Message {
        Message {
            form: 0x100 + n,
            title: format!("Page {n}"),
            text: format!("Text {n}"),
        }
    }

    fn manual(at: Option<Message>) -> (Ui, TutorialMenu, bool) {
        let mut ui = test_support::ui();
        let mut m = TutorialMenu::new(0, 0);
        m.menu = test_support::load(&mut ui, XML, &mut m);
        let ok = m.open_manual(&mut ui, at, (0..3).map(page).collect());
        (ui, m, ok)
    }

    /// The start menu's Help: the manual from its first page, "1/3",
    /// Next clickable, Previous not; the buttons' own words.
    #[test]
    fn the_manual_opens_on_its_first_page() {
        let (mut ui, m, ok) = manual(None);
        assert!(ok);
        let tile = |i: usize| m.tiles[i].unwrap();
        assert_eq!(ui.string(tile(tile::TITLE), t::STRING).unwrap(), "Page 0");
        assert_eq!(ui.string(tile(tile::PAGE), t::STRING).unwrap(), "1/3");
        assert_eq!(ui.string(tile(tile::NEXT), t::STRING).unwrap(), "Next");
        assert_eq!(
            ui.string(tile(tile::PREVIOUS), t::STRING).unwrap(),
            "Previous"
        );
        assert_eq!(ui.number(tile(tile::NEXT), t::VISIBLE), 1.0);
        assert_eq!(ui.number(tile(tile::PAGE), t::VISIBLE), 1.0);
        assert_eq!(ui.number(tile(tile::NEXT), t::TARGET), 1.0);
        assert_eq!(ui.number(tile(tile::PREVIOUS), t::TARGET), 0.0);
        assert_eq!((m.page, m.pages), (0, 3));
    }

    /// Next and Previous turn the pages, and stop at the ends.
    #[test]
    fn next_and_previous_turn_the_pages() {
        let (mut ui, mut m, _) = manual(Some(page(1)));
        let tile = |m: &TutorialMenu, i: usize| m.tiles[i].unwrap();
        assert_eq!(ui.string(tile(&m, tile::PAGE), t::STRING).unwrap(), "2/3");
        m.click(&mut ui, tile::NEXT as i32, None, 0.0);
        assert_eq!(
            ui.string(tile(&m, tile::TITLE), t::STRING).unwrap(),
            "Page 2"
        );
        assert_eq!(
            ui.string(tile(&m, tile::TEXT), t::STRING).unwrap(),
            "Text 2"
        );
        assert_eq!(ui.string(tile(&m, tile::PAGE), t::STRING).unwrap(), "3/3");
        assert_eq!(ui.number(tile(&m, tile::NEXT), t::TARGET), 0.0);
        m.click(&mut ui, tile::NEXT as i32, None, 0.0);
        assert_eq!(m.page, 2);
        for _ in 0..3 {
            m.click(&mut ui, tile::PREVIOUS as i32, None, 0.0);
        }
        assert_eq!(m.page, 0);
        assert_eq!(
            ui.string(tile(&m, tile::TITLE), t::STRING).unwrap(),
            "Page 0"
        );
        assert!(!m.closed);
    }

    /// A message not in the manual: shown without pages.
    #[test]
    fn a_message_not_in_the_manual_has_no_pages() {
        let (mut ui, m, ok) = manual(Some(page(9)));
        assert!(ok);
        let tile = |i: usize| m.tiles[i].unwrap();
        assert_eq!(ui.string(tile(tile::TITLE), t::STRING).unwrap(), "Page 9");
        assert_eq!(ui.number(tile(tile::PAGE), t::VISIBLE), 0.0);
        assert_eq!((m.page, m.pages), (0, 0));
        assert_eq!(ui.number(tile(tile::NEXT), t::TARGET), 0.0);
    }

    /// No message and an empty manual: it closes again.
    #[test]
    fn nothing_to_show_closes_it() {
        let mut ui = test_support::ui();
        let mut m = TutorialMenu::new(0, 0);
        m.menu = test_support::load(&mut ui, XML, &mut m);
        assert!(!m.open_manual(&mut ui, None, Vec::new()));
        assert!(m.closed);
    }
}
